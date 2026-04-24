//! Application state machine for the NQRustBackup installer.

#![allow(dead_code)]

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::installer::executor::Phase;

/// Where to get NQRustBackup binaries from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InstallSource {
    /// Use the upstream-compatible Bareos community packages.
    ///
    /// Proven config-compatible with NQRustBackup's `debian/` tree by the
    /// P2V eval. This is the fast path and is the default for testing.
    #[default]
    UpstreamCompat,
    /// Build NQRustBackup `.deb` packages from the source tree this installer
    /// lives in, using `dpkg-buildpackage`. Slow, authentic.
    BuildFromSource,
    /// Don't install server packages; just (re)write configuration and start
    /// services. Use when the binaries are already present.
    ConfigureOnly,
}

impl InstallSource {
    pub const ALL: [InstallSource; 3] = [
        InstallSource::UpstreamCompat,
        InstallSource::BuildFromSource,
        InstallSource::ConfigureOnly,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            InstallSource::UpstreamCompat => "Upstream-compatible packages",
            InstallSource::BuildFromSource => "Build from source (.deb)",
            InstallSource::ConfigureOnly => "Configure only (binaries already installed)",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            InstallSource::UpstreamCompat => {
                "Install Bareos community packages — config-compatible with NQRustBackup. ~5 min."
            }
            InstallSource::BuildFromSource => {
                "Build NQRustBackup debs from this repo with dpkg-buildpackage. ~30-60 min."
            }
            InstallSource::ConfigureOnly => {
                "Skip package install; write /etc/bareos configs and start services."
            }
        }
    }
}

/// Which components to install.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InstallProfile {
    /// Director + Storage + FileDaemon + Catalog + WebUI on one host.
    #[default]
    AllInOne,
    /// Director + Storage + Catalog (no FD, no WebUI) — classic backup server.
    ServerOnly,
    /// FileDaemon only (a client host).
    ClientOnly,
}

impl InstallProfile {
    pub const ALL: [InstallProfile; 3] = [
        InstallProfile::AllInOne,
        InstallProfile::ServerOnly,
        InstallProfile::ClientOnly,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            InstallProfile::AllInOne => "All-in-One (recommended for testing)",
            InstallProfile::ServerOnly => "Server (Director + Storage + Catalog)",
            InstallProfile::ClientOnly => "Client (FileDaemon only)",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            InstallProfile::AllInOne => "DIR + SD + FD + PostgreSQL + WebUI on this host. Single host, full stack.",
            InstallProfile::ServerOnly => "Backup server only. Clients install separately with ClientOnly.",
            InstallProfile::ClientOnly => "File Daemon only. Point it at a remote DIR with `nqrustbackup-installer configure-client`.",
        }
    }

    pub fn installs_director(&self) -> bool {
        matches!(self, InstallProfile::AllInOne | InstallProfile::ServerOnly)
    }

    pub fn installs_storage(&self) -> bool {
        matches!(self, InstallProfile::AllInOne | InstallProfile::ServerOnly)
    }

    pub fn installs_filedaemon(&self) -> bool {
        matches!(self, InstallProfile::AllInOne | InstallProfile::ClientOnly)
    }

    pub fn installs_webui(&self) -> bool {
        matches!(self, InstallProfile::AllInOne)
    }

    pub fn installs_catalog(&self) -> bool {
        matches!(self, InstallProfile::AllInOne | InstallProfile::ServerOnly)
    }
}

#[derive(Debug, Clone)]
pub struct InstallConfig {
    pub source: InstallSource,
    pub profile: InstallProfile,
    pub data_dir: PathBuf,
    pub config_dir: PathBuf,
    pub storage_dir: PathBuf,
    pub webui_port: u16,
    pub director_address: String,
    pub dry_run: bool,
}

impl Default for InstallConfig {
    fn default() -> Self {
        Self {
            source: InstallSource::default(),
            profile: InstallProfile::default(),
            data_dir: PathBuf::from("/var/lib/bareos"),
            config_dir: PathBuf::from("/etc/bareos"),
            storage_dir: PathBuf::from("/var/lib/bareos/storage"),
            webui_port: 9100,
            director_address: "localhost".to_string(),
            dry_run: false,
        }
    }
}

/// Which screen the TUI is currently on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Screen {
    #[default]
    Welcome,
    Preflight,
    SourceSelect,
    ProfileSelect,
    Config,
    Progress,
    Verify,
    Complete,
    Error,
}

impl Screen {
    pub fn title(&self) -> &'static str {
        match self {
            Screen::Welcome => "Welcome",
            Screen::Preflight => "Pre-flight checks",
            Screen::SourceSelect => "Install source",
            Screen::ProfileSelect => "Install profile",
            Screen::Config => "Configuration",
            Screen::Progress => "Installing",
            Screen::Verify => "Verification",
            Screen::Complete => "Complete",
            Screen::Error => "Error",
        }
    }

    pub fn step(&self) -> usize {
        match self {
            Screen::Welcome => 1,
            Screen::Preflight => 2,
            Screen::SourceSelect => 3,
            Screen::ProfileSelect => 4,
            Screen::Config => 5,
            Screen::Progress => 6,
            Screen::Verify => 7,
            Screen::Complete => 8,
            Screen::Error => 0,
        }
    }
}

pub const TOTAL_STEPS: usize = 8;

/// A single pre-flight check outcome, shown in the Preflight screen.
#[derive(Debug, Clone)]
pub struct PreflightCheck {
    pub name: String,
    pub status: CheckStatus,
    pub detail: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckStatus {
    Pending,
    Running,
    Ok,
    Warn,
    Fail,
}

/// Shared log ring passed to executor tasks; the Progress screen reads this.
#[derive(Debug, Default, Clone)]
pub struct LogRing {
    pub inner: Arc<Mutex<Vec<(chrono::DateTime<chrono::Local>, LogLevel, String)>>>,
    pub max: usize,
}

impl LogRing {
    pub fn new(max: usize) -> Self {
        Self {
            inner: Arc::new(Mutex::new(Vec::with_capacity(max))),
            max,
        }
    }

    pub fn push(&self, level: LogLevel, msg: impl Into<String>) {
        let mut g = self.inner.lock().unwrap();
        g.push((chrono::Local::now(), level, msg.into()));
        let over = g.len().saturating_sub(self.max);
        if over > 0 {
            g.drain(0..over);
        }
    }

    pub fn snapshot(&self) -> Vec<(chrono::DateTime<chrono::Local>, LogLevel, String)> {
        self.inner.lock().unwrap().clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Ok,
    Warn,
    Err,
}

#[derive(Debug)]
pub struct App {
    pub screen: Screen,
    pub config: InstallConfig,
    pub preflight: Vec<PreflightCheck>,
    pub source_idx: usize,
    pub profile_idx: usize,
    pub config_focus: usize,
    /// Progress: 0..=total_phases; current_phase is which one is in flight.
    pub phases: Vec<(Phase, CheckStatus)>,
    pub current_phase: usize,
    pub verify: Vec<PreflightCheck>,
    pub log: LogRing,
    pub error_message: Option<String>,
    pub spinner_tick: usize,
    pub should_quit: bool,
    pub complete_url: Option<String>,
    /// If true, executor is running and we should not let the user skip ahead.
    pub executor_running: bool,
    /// Shared status vec that the executor task updates per phase.
    pub exec_status: Option<std::sync::Arc<std::sync::Mutex<Vec<CheckStatus>>>>,
}

impl App {
    pub fn new(config: InstallConfig) -> Self {
        Self {
            screen: Screen::Welcome,
            config,
            preflight: Vec::new(),
            source_idx: 0,
            profile_idx: 0,
            config_focus: 0,
            phases: Vec::new(),
            current_phase: 0,
            verify: Vec::new(),
            log: LogRing::new(2000),
            error_message: None,
            spinner_tick: 0,
            should_quit: false,
            complete_url: None,
            executor_running: false,
            exec_status: None,
        }
    }

    pub fn selected_source(&self) -> InstallSource {
        InstallSource::ALL[self.source_idx]
    }

    pub fn selected_profile(&self) -> InstallProfile {
        InstallProfile::ALL[self.profile_idx]
    }

    pub fn tick(&mut self) {
        self.spinner_tick = self.spinner_tick.wrapping_add(1);
    }
}
