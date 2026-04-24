//! PostgreSQL catalog bootstrap. Assumes the `postgresql` package is
//! installed and the cluster is running.

use anyhow::Result;

use crate::app::{InstallConfig, LogLevel, LogRing};
use crate::installer::executor::sudo_run_logged;

pub async fn bootstrap_catalog(cfg: &InstallConfig, log: &LogRing) -> Result<()> {
    if !cfg.profile.installs_catalog() {
        log.push(LogLevel::Info, "profile has no catalog — skipping");
        return Ok(());
    }

    log.push(LogLevel::Info, "creating catalog DB (bareos scripts)");

    // These scripts must run as the postgres OS user (peer auth).
    let cmds = [
        "sudo -u postgres /usr/lib/bareos/scripts/create_bareos_database postgresql",
        "sudo -u postgres /usr/lib/bareos/scripts/make_bareos_tables postgresql",
        "sudo -u postgres /usr/lib/bareos/scripts/grant_bareos_privileges postgresql",
    ];
    for c in cmds {
        sudo_run_logged(&["sh", "-c", c], log, cfg.dry_run).await?;
    }
    Ok(())
}
