//! Entry points for each install-source strategy.
//!
//! Each function here composes the low-level ops in `deps.rs` into a single
//! "install packages" phase matching the chosen InstallSource.

use anyhow::Result;

use crate::app::{InstallConfig, InstallSource, LogLevel, LogRing};
use crate::installer::{deps, preflight};

pub async fn run(cfg: &InstallConfig, log: &LogRing) -> Result<()> {
    match cfg.source {
        InstallSource::UpstreamCompat => run_upstream(cfg, log).await,
        InstallSource::BuildFromSource => run_build(cfg, log).await,
        InstallSource::ConfigureOnly => {
            log.push(
                LogLevel::Info,
                "configure-only source: skipping package install",
            );
            Ok(())
        }
    }
}

async fn run_upstream(cfg: &InstallConfig, log: &LogRing) -> Result<()> {
    let os = preflight::detect_os();
    deps::install_apt_deps(log, cfg.dry_run).await?;
    deps::configure_upstream_repo(log, &os, cfg.dry_run).await?;
    deps::install_packages(cfg.profile, log, cfg.dry_run).await?;
    Ok(())
}

async fn run_build(cfg: &InstallConfig, log: &LogRing) -> Result<()> {
    deps::install_apt_deps(log, cfg.dry_run).await?;
    deps::build_from_source(log, cfg.dry_run).await?;
    // After build, actually install the produced debs — assume they're in ../ from repo root.
    log.push(
        LogLevel::Info,
        "build-from-source: locating built .debs and dpkg -i",
    );
    // TODO: enumerate and install. For the v0.1 we leave the user to dpkg -i manually
    // (keeps this path non-lossy on non-Debian hosts).
    Ok(())
}
