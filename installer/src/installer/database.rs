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

    // Idempotent: skip the whole step if the bareos catalog already has tables.
    // This makes re-running the installer (e.g. with --source configure-only
    // to apply later config changes) safe — the create scripts exit 3 when the
    // database already exists, which would otherwise abort the run.
    log.push(LogLevel::Info, "checking for existing catalog");
    let probe = "sudo -u postgres psql -tAc 'SELECT 1 FROM pg_database WHERE datname=$$bareos$$' 2>/dev/null | grep -q 1";
    let exists = sudo_run_logged(&["sh", "-c", probe], log, cfg.dry_run)
        .await
        .is_ok();
    if exists {
        log.push(
            LogLevel::Info,
            "catalog already present — skipping create/grant",
        );
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
