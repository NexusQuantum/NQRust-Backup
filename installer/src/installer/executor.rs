//! Orchestrates the install phases.
//!
//! Each phase is a named async task that the TUI drives one-by-one. The
//! headless CLI runs the same list imperatively.

use std::process::Stdio;

use anyhow::{bail, Context, Result};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

use crate::app::{InstallConfig, InstallProfile, InstallSource, LogLevel, LogRing};

use super::{config, database, services, source, verify, webui};

#[derive(Debug, Clone)]
pub struct Phase {
    pub ordinal: usize,
    pub name: String,
    pub kind: PhaseKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhaseKind {
    InstallSource,
    BootstrapCatalog,
    RenderConfig,
    EnableServices,
    DeployWebui,
    Reload,
    Verify,
}

pub struct Task {
    pub phase: Phase,
}

impl Task {
    pub async fn run(&self, cfg: &InstallConfig, log: &LogRing) -> Result<()> {
        log.push(
            LogLevel::Info,
            format!("▶ phase {}: {}", self.phase.ordinal, self.phase.name),
        );
        let res: Result<()> = match self.phase.kind {
            PhaseKind::InstallSource => source::run(cfg, log).await,
            PhaseKind::BootstrapCatalog => database::bootstrap_catalog(cfg, log).await,
            PhaseKind::RenderConfig => config::render_and_install(cfg, log).await,
            PhaseKind::EnableServices => services::enable_and_start(cfg, log).await,
            PhaseKind::DeployWebui => webui::setup(cfg, log).await,
            PhaseKind::Reload => services::reload_director(cfg, log).await,
            PhaseKind::Verify => {
                let _ = verify::run(cfg, log).await?;
                Ok(())
            }
        };
        match &res {
            Ok(()) => log.push(
                LogLevel::Ok,
                format!("✓ phase {}: {}", self.phase.ordinal, self.phase.name),
            ),
            Err(e) => log.push(
                LogLevel::Err,
                format!(
                    "✗ phase {}: {} — {e:#}",
                    self.phase.ordinal, self.phase.name
                ),
            ),
        }
        res
    }
}

pub fn planned_phases(src: InstallSource, profile: InstallProfile) -> Vec<Phase> {
    let mut phases = Vec::new();
    let mut n = 0;
    let push = |phases: &mut Vec<Phase>, n: &mut usize, name: &str, kind: PhaseKind| {
        *n += 1;
        phases.push(Phase {
            ordinal: *n,
            name: name.to_string(),
            kind,
        });
    };

    match src {
        InstallSource::UpstreamCompat => push(
            &mut phases,
            &mut n,
            "Install upstream-compat packages",
            PhaseKind::InstallSource,
        ),
        InstallSource::BuildFromSource => push(
            &mut phases,
            &mut n,
            "Build + install .deb from source",
            PhaseKind::InstallSource,
        ),
        InstallSource::ConfigureOnly => {}
    }

    if profile.installs_catalog() {
        push(
            &mut phases,
            &mut n,
            "Bootstrap PostgreSQL catalog",
            PhaseKind::BootstrapCatalog,
        );
    }

    push(
        &mut phases,
        &mut n,
        "Render NQRustBackup config fragments",
        PhaseKind::RenderConfig,
    );
    push(
        &mut phases,
        &mut n,
        "Enable + start services",
        PhaseKind::EnableServices,
    );

    if profile.installs_webui() {
        push(&mut phases, &mut n, "Deploy WebUI", PhaseKind::DeployWebui);
    }

    if profile.installs_director() {
        push(
            &mut phases,
            &mut n,
            "Reload director (pick up new fragments)",
            PhaseKind::Reload,
        );
    }

    push(&mut phases, &mut n, "Verify", PhaseKind::Verify);
    phases
}

pub fn build_tasks(cfg: &InstallConfig) -> Vec<Task> {
    planned_phases(cfg.source, cfg.profile)
        .into_iter()
        .map(|phase| Task { phase })
        .collect()
}

/// Run a command under sudo (or direct if already root), streaming stdout/stderr into the log ring.
pub async fn sudo_run_logged(cmd: &[&str], log: &LogRing, dry_run: bool) -> Result<()> {
    if dry_run {
        log.push(
            LogLevel::Info,
            format!("(dry-run) would run: {}", render_cmd(cmd)),
        );
        return Ok(());
    }

    let (program, args) = build_sudo_cmd(cmd);
    log.push(
        LogLevel::Info,
        format!("$ {} {}", program, args.join(" ")),
    );

    let mut child = Command::new(program)
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("spawning {}", render_cmd(cmd)))?;

    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    let log_out = log.clone();
    let log_err = log.clone();

    let h_out = tokio::spawn(async move {
        if let Some(s) = stdout {
            let mut r = BufReader::new(s).lines();
            while let Ok(Some(line)) = r.next_line().await {
                log_out.push(LogLevel::Info, line);
            }
        }
    });
    let h_err = tokio::spawn(async move {
        if let Some(s) = stderr {
            let mut r = BufReader::new(s).lines();
            while let Ok(Some(line)) = r.next_line().await {
                log_err.push(LogLevel::Warn, line);
            }
        }
    });

    let status = child.wait().await.context("waiting for child")?;
    let _ = h_out.await;
    let _ = h_err.await;

    if !status.success() {
        bail!(
            "command failed with status {}: {}",
            status,
            render_cmd(cmd)
        );
    }
    Ok(())
}

/// Variant without dry-run support (used when we've already decided to run).
pub async fn sudo_run(cmd: &[&str], log: &LogRing) -> Result<()> {
    sudo_run_logged(cmd, log, false).await
}

fn build_sudo_cmd(cmd: &[&str]) -> (String, Vec<String>) {
    // If we're root, skip sudo entirely.
    let am_root = unsafe { libc_geteuid() == 0 };
    if am_root {
        let program = cmd[0].to_string();
        let args = cmd[1..].iter().map(|s| s.to_string()).collect();
        return (program, args);
    }

    let mut args: Vec<String> = vec!["-n".to_string()];
    for s in cmd {
        args.push((*s).to_string());
    }
    ("sudo".to_string(), args)
}

#[link(name = "c")]
extern "C" {
    fn geteuid() -> u32;
}

unsafe fn libc_geteuid() -> u32 {
    geteuid()
}

fn render_cmd(cmd: &[&str]) -> String {
    cmd.iter()
        .map(|s| shell_escape_if_needed(s))
        .collect::<Vec<_>>()
        .join(" ")
}

fn shell_escape_if_needed(s: &str) -> String {
    if s.chars().any(|c| c.is_whitespace() || c == '\'' || c == '"' || c == '$') {
        format!("'{}'", s.replace('\'', "'\\''"))
    } else {
        s.to_string()
    }
}

