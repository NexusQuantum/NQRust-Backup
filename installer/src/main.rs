//! NQRustBackup installer — TUI entry point.

// Stylistic clippy lints we deliberately don't chase — they shift between
// stable rust versions and are not correctness issues. Prevents "build broken
// by toolchain upgrade" surprises in CI.
#![allow(
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::single_match,
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::module_inception
)]

mod app;
mod installer;
mod theme;
mod ui;

use std::io;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::app::{App, InstallConfig, InstallProfile, InstallSource};
use crate::installer::executor;

#[derive(Parser)]
#[command(name = "nqrustbackup-installer")]
#[command(version)]
#[command(
    about = "NQRustBackup Installer — TUI for deploying Director/Storage/FileDaemon/Catalog/WebUI"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the interactive TUI installer (default).
    Tui,
    /// Non-interactive install with explicit flags (CI / scripted).
    Install {
        #[arg(long, value_enum, default_value = "upstream-compat")]
        source: CliSource,
        #[arg(long, value_enum, default_value = "all-in-one")]
        profile: CliProfile,
        #[arg(long, default_value = "/var/lib/bareos")]
        data_dir: PathBuf,
        #[arg(long, default_value = "/etc/bareos")]
        config_dir: PathBuf,
        #[arg(long, default_value = "/var/lib/bareos/storage")]
        storage_dir: PathBuf,
        #[arg(long, default_value = "9100")]
        webui_port: u16,
        #[arg(long, default_value = "localhost")]
        director_address: String,
        /// Print what would run instead of running it.
        #[arg(long)]
        dry_run: bool,
    },
    /// Print the resolved plan for a given source/profile and exit.
    Plan {
        #[arg(long, value_enum, default_value = "upstream-compat")]
        source: CliSource,
        #[arg(long, value_enum, default_value = "all-in-one")]
        profile: CliProfile,
    },
    /// Run an end-to-end backup + restore round-trip against an installed
    /// NQRustBackup server. Uses the bundled `backup-bareos-fd` job, restores
    /// to a temp dir, and SHA-256 spot-checks N restored files vs originals.
    TestRoundtrip {
        /// Job to run (default: bundled SelfTest fileset on bareos-fd)
        #[arg(long, default_value = "backup-bareos-fd")]
        job: String,
        /// Pool to label new volume in
        #[arg(long, default_value = "Full")]
        pool: String,
        /// Directory to restore into (auto-named under /tmp if unset)
        #[arg(long)]
        restore_dir: Option<PathBuf>,
        /// Number of restored files to SHA-256 vs originals
        #[arg(long, default_value = "10")]
        samples: usize,
        /// Don't delete the restore dir on success
        #[arg(long)]
        keep: bool,
    },
}

#[derive(Copy, Clone, ValueEnum)]
enum CliSource {
    UpstreamCompat,
    BuildFromSource,
    ConfigureOnly,
}

impl From<CliSource> for InstallSource {
    fn from(v: CliSource) -> Self {
        match v {
            CliSource::UpstreamCompat => InstallSource::UpstreamCompat,
            CliSource::BuildFromSource => InstallSource::BuildFromSource,
            CliSource::ConfigureOnly => InstallSource::ConfigureOnly,
        }
    }
}

#[derive(Copy, Clone, ValueEnum)]
enum CliProfile {
    AllInOne,
    ServerOnly,
    ClientOnly,
}

impl From<CliProfile> for InstallProfile {
    fn from(v: CliProfile) -> Self {
        match v {
            CliProfile::AllInOne => InstallProfile::AllInOne,
            CliProfile::ServerOnly => InstallProfile::ServerOnly,
            CliProfile::ClientOnly => InstallProfile::ClientOnly,
        }
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command.unwrap_or(Commands::Tui) {
        Commands::Tui => run_tui().await,
        Commands::Install {
            source,
            profile,
            data_dir,
            config_dir,
            storage_dir,
            webui_port,
            director_address,
            dry_run,
        } => {
            let cfg = InstallConfig {
                source: source.into(),
                profile: profile.into(),
                data_dir,
                config_dir,
                storage_dir,
                webui_port,
                director_address,
                dry_run,
            };
            run_headless(cfg).await
        }
        Commands::TestRoundtrip {
            job,
            pool,
            restore_dir,
            samples,
            keep,
        } => {
            tracing_subscriber::fmt()
                .with_env_filter(
                    tracing_subscriber::EnvFilter::try_from_default_env()
                        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
                )
                .init();
            let defaults = installer::roundtrip::RoundtripOptions::default();
            let opts = installer::roundtrip::RoundtripOptions {
                job_name: job,
                volume_pool: pool,
                restore_dir: restore_dir.unwrap_or(defaults.restore_dir),
                samples,
                keep_restore_dir: keep,
                ..defaults
            };
            let log = app::LogRing::new(2000);
            // Mirror logs to stdout for the CLI
            let log_clone = log.clone();
            std::thread::spawn(move || {
                let mut last_seen = 0usize;
                loop {
                    let snap = log_clone.snapshot();
                    if snap.len() > last_seen {
                        for entry in &snap[last_seen..] {
                            let (ts, lvl, msg) = entry;
                            let tag = match lvl {
                                app::LogLevel::Info => "INFO",
                                app::LogLevel::Ok => "OK  ",
                                app::LogLevel::Warn => "WARN",
                                app::LogLevel::Err => "ERR ",
                            };
                            println!("{}  {tag}  {msg}", ts.format("%H:%M:%S"));
                        }
                        last_seen = snap.len();
                    }
                    std::thread::sleep(std::time::Duration::from_millis(120));
                }
            });
            let report =
                match tokio::task::spawn_blocking(move || installer::roundtrip::run(&opts, &log))
                    .await?
                {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("\nROUND-TRIP FAILED: {e:#}");
                        std::process::exit(1);
                    }
                };
            // Tiny summary
            println!();
            println!("====================================================");
            println!(
                " Round-trip: {}",
                if report.passed() { "PASS" } else { "FAIL" }
            );
            println!(
                "   backup  jobid={} files={} bytes={} secs={}",
                report.backup_jobid,
                report.files_backed_up,
                report.bytes_backed_up,
                report.backup_secs
            );
            println!(
                "   restore jobid={} files={} bytes={} secs={}",
                report.restore_jobid,
                report.files_restored,
                report.bytes_restored,
                report.restore_secs
            );
            println!(
                "   sha256: {}/{} samples matched",
                report.samples_matched, report.samples_checked
            );
            if report.passed() {
                Ok(())
            } else {
                std::process::exit(2);
            }
        }
        Commands::Plan { source, profile } => {
            let src: InstallSource = source.into();
            let prof: InstallProfile = profile.into();
            println!("Source:  {}", src.name());
            println!("Profile: {}", prof.name());
            println!();
            for phase in executor::planned_phases(src, prof) {
                println!("  [{:>2}] {}", phase.ordinal, phase.name);
            }
            Ok(())
        }
    }
}

async fn run_headless(cfg: InstallConfig) -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let log = app::LogRing::new(2000);
    let phases = executor::planned_phases(cfg.source, cfg.profile);
    println!(
        "nqrustbackup-installer: {} → {}  ({} phases)",
        cfg.source.name(),
        cfg.profile.name(),
        phases.len()
    );
    for phase in &phases {
        println!("  [{:>2}] {}", phase.ordinal, phase.name);
    }
    println!();

    let mut tasks = executor::build_tasks(&cfg);
    for (i, task) in tasks.iter_mut().enumerate() {
        let phase_name = task.phase.name.clone();
        println!(">> phase {}/{}: {}", i + 1, phases.len(), phase_name);
        match task.run(&cfg, &log).await {
            Ok(()) => println!("   ok"),
            Err(e) => {
                eprintln!("   FAIL: {e:#}");
                return Err(e);
            }
        }
    }

    println!();
    println!("Install complete. Verify at:");
    if cfg.profile.installs_webui() {
        println!(
            "  WebUI: http://{}:{}",
            cfg.director_address, cfg.webui_port
        );
    }
    println!("  bconsole (on this host): `sudo bconsole`");
    Ok(())
}

async fn run_tui() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = tui_loop(&mut terminal).await;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}

async fn tui_loop<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    let mut app = App::new(InstallConfig::default());

    let tick_rate = Duration::from_millis(120);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::ZERO);

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                // Global quit
                if matches!(key.code, KeyCode::Char('q')) && !app.executor_running {
                    app.should_quit = true;
                }
                if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    app.should_quit = true;
                }

                ui::handle_key(&mut app, key).await;
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
