//! Install OS-level dependencies and configure the apt repository for
//! NQRustBackup (or its upstream-compatible stand-in).

use anyhow::{bail, Context, Result};

use crate::app::{LogLevel, LogRing};
use crate::installer::preflight::{detect_os, OsInfo};

use super::executor::{sudo_run, sudo_run_logged};

pub async fn install_apt_deps(log: &LogRing, dry_run: bool) -> Result<()> {
    let os = detect_os();
    if !os.is_debian_family {
        log.push(
            LogLevel::Warn,
            format!(
                "host is {} (not Debian/Ubuntu); skipping apt install",
                os.pretty_name
            ),
        );
        if dry_run {
            return Ok(());
        }
        bail!("apt-based install only supports Debian/Ubuntu. Try --dry-run or use BuildFromSource.");
    }

    log.push(LogLevel::Info, "apt-get update");
    sudo_run_logged(&["apt-get", "update", "-y"], log, dry_run).await?;

    log.push(LogLevel::Info, "installing prerequisites");
    let deps = [
        "curl",
        "ca-certificates",
        "gnupg",
        "postgresql",
        "postgresql-contrib",
        "openssh-server",
    ];
    let mut cmd: Vec<&str> = vec!["apt-get", "install", "-y", "--no-install-recommends"];
    cmd.extend(deps);
    sudo_run_logged(&cmd, log, dry_run).await?;

    Ok(())
}

pub async fn configure_upstream_repo(log: &LogRing, os: &OsInfo, dry_run: bool) -> Result<()> {
    log.push(LogLevel::Info, "configuring upstream-compat apt repo");
    let distro = format!("xUbuntu_{}", os.version_id);
    let gpg_url = format!("https://download.bareos.org/current/{distro}/Release.key");
    let repo = format!("https://download.bareos.org/current/{distro}/");

    let script = format!(
        r#"
set -eu
mkdir -p /etc/apt/keyrings
curl -fsSL '{gpg_url}' | gpg --dearmor -o /etc/apt/keyrings/nqrustbackup-upstream.gpg
echo 'deb [signed-by=/etc/apt/keyrings/nqrustbackup-upstream.gpg] {repo} /' > /etc/apt/sources.list.d/nqrustbackup-upstream.list
apt-get update -y
"#
    );
    sudo_run_logged(&["sh", "-c", &script], log, dry_run).await?;
    Ok(())
}

/// Install bareos packages (upstream-compatible stand-in for NQRustBackup).
///
/// When NQRustBackup publishes its own apt repo, this function only needs its
/// repo URLs swapped — package names match.
pub async fn install_packages(
    profile: crate::app::InstallProfile,
    log: &LogRing,
    dry_run: bool,
) -> Result<()> {
    let mut pkgs: Vec<&str> = Vec::new();

    if profile.installs_director() {
        pkgs.push("bareos-director");
    }
    if profile.installs_storage() {
        pkgs.push("bareos-storage");
    }
    if profile.installs_filedaemon() {
        pkgs.push("bareos-filedaemon");
    }
    if profile.installs_catalog() {
        pkgs.push("bareos-database-postgresql");
    }
    pkgs.push("bareos-bconsole");
    if profile.installs_webui() {
        pkgs.push("bareos-webui");
    }

    // Preseed anything that might prompt.
    let preseed = r#"postfix postfix/main_mailer_type select No configuration
postfix postfix/mailname string nqrustbackup.local
bareos-database-common bareos-database-common/dbconfig-install boolean false
bareos-database-common bareos-database-common/install-error select abort
dbconfig-common dbconfig-common/dbconfig-install boolean false
"#;

    log.push(LogLevel::Info, format!("installing: {}", pkgs.join(" ")));

    if dry_run {
        log.push(LogLevel::Info, "(dry-run) would run apt-get install with preseed");
        return Ok(());
    }

    // Write preseed to a tempfile on the host (under sudo).
    let preseed_cmd = format!(
        "cat > /tmp/nqrb-preseed.txt <<'__EOF__'\n{preseed}__EOF__\ndebconf-set-selections /tmp/nqrb-preseed.txt"
    );
    sudo_run(&["sh", "-c", &preseed_cmd], log).await?;

    // Disable needrestart prompts
    let needrestart = r#"mkdir -p /etc/needrestart/conf.d
cat > /etc/needrestart/conf.d/nqrb.conf <<'__EOF__'
$nrconf{restart} = "a";
$nrconf{kernelhints} = -1;
__EOF__
"#;
    sudo_run(&["sh", "-c", needrestart], log).await?;

    let mut cmd: Vec<&str> = vec![
        "env",
        "DEBIAN_FRONTEND=noninteractive",
        "NEEDRESTART_MODE=a",
        "apt-get",
        "install",
        "-y",
        "-o",
        "Dpkg::Options::=--force-confold",
    ];
    cmd.extend(&pkgs);
    sudo_run_logged(&cmd, log, dry_run).await?;

    Ok(())
}

pub async fn build_from_source(log: &LogRing, dry_run: bool) -> Result<()> {
    log.push(
        LogLevel::Info,
        "build-from-source path: running `apt build-dep .` + dpkg-buildpackage in repo root",
    );
    let repo_root = find_repo_root().context("could not find NQRustBackup repo root")?;
    let repo_root_str = repo_root.to_string_lossy().to_string();

    let build_script = format!(
        r#"set -eu
cd '{repo_root_str}'
apt-get update -y
apt build-dep -y .
dpkg-buildpackage -us -uc -b -j$(nproc)
"#
    );
    sudo_run_logged(&["sh", "-c", &build_script], log, dry_run).await?;

    log.push(
        LogLevel::Info,
        format!(
            "build-from-source: debs emitted under {}",
            repo_root.parent().unwrap_or(&repo_root).display()
        ),
    );
    Ok(())
}

fn find_repo_root() -> Option<std::path::PathBuf> {
    // Walk up from the exe dir looking for debian/control.src.
    let mut p = std::env::current_exe().ok()?.canonicalize().ok()?;
    for _ in 0..6 {
        if !p.pop() {
            break;
        }
        if p.join("debian/control.src").exists() {
            return Some(p);
        }
    }
    // Fall back to CWD
    let cwd = std::env::current_dir().ok()?;
    if cwd.join("debian/control.src").exists() {
        Some(cwd)
    } else {
        None
    }
}
