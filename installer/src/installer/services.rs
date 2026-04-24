//! Enable and start NQRustBackup / bareos systemd units.
//!
//! Package unit names on current Bareos packages are `bareos-director`,
//! `bareos-storage`, `bareos-filedaemon` (with `bareos-dir` / `-sd` / `-fd` as
//! aliases — `systemctl enable` refuses the alias form).

use anyhow::Result;

use crate::app::{InstallConfig, LogLevel, LogRing};
use crate::installer::executor::sudo_run_logged;

pub async fn enable_and_start(cfg: &InstallConfig, log: &LogRing) -> Result<()> {
    let mut units: Vec<&str> = Vec::new();
    if cfg.profile.installs_director() {
        units.push("bareos-director");
    }
    if cfg.profile.installs_storage() {
        units.push("bareos-storage");
    }
    if cfg.profile.installs_filedaemon() {
        units.push("bareos-filedaemon");
    }
    if units.is_empty() {
        log.push(LogLevel::Info, "no daemons to start for this profile");
        return Ok(());
    }

    log.push(LogLevel::Info, format!("enabling: {}", units.join(" ")));
    let mut cmd: Vec<&str> = vec!["systemctl", "enable", "--now"];
    cmd.extend(&units);
    sudo_run_logged(&cmd, log, cfg.dry_run).await?;
    Ok(())
}

pub async fn reload_director(cfg: &InstallConfig, log: &LogRing) -> Result<()> {
    if !cfg.profile.installs_director() {
        return Ok(());
    }
    let script = r#"if command -v bconsole >/dev/null 2>&1; then
    echo -e 'reload\nquit' | bconsole || true
fi"#;
    sudo_run_logged(&["sh", "-c", script], log, cfg.dry_run).await?;
    Ok(())
}
