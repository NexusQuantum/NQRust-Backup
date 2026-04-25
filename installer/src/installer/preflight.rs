//! Pre-flight checks — detect OS, privileges, disk space, network.

#![allow(dead_code)]

use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::app::{CheckStatus, PreflightCheck};

pub struct OsInfo {
    pub id: String,
    pub version_id: String,
    pub pretty_name: String,
    pub is_debian_family: bool,
}

pub fn detect_os() -> OsInfo {
    let mut id = "unknown".to_string();
    let mut version_id = "0".to_string();
    let mut pretty = "Unknown".to_string();
    if let Ok(s) = std::fs::read_to_string("/etc/os-release") {
        for line in s.lines() {
            if let Some(v) = line.strip_prefix("ID=") {
                id = unquote(v);
            } else if let Some(v) = line.strip_prefix("VERSION_ID=") {
                version_id = unquote(v);
            } else if let Some(v) = line.strip_prefix("PRETTY_NAME=") {
                pretty = unquote(v);
            }
        }
    }
    let is_debian_family = matches!(id.as_str(), "debian" | "ubuntu");
    OsInfo {
        id,
        version_id,
        pretty_name: pretty,
        is_debian_family,
    }
}

fn unquote(s: &str) -> String {
    s.trim().trim_matches('"').to_string()
}

pub fn run_all_checks(install_dir: &Path, storage_dir: &Path) -> Vec<PreflightCheck> {
    let mut out = Vec::new();

    // OS
    let os = detect_os();
    let (os_status, os_detail) = if os.is_debian_family {
        (CheckStatus::Ok, format!("{} (supported)", os.pretty_name))
    } else {
        (
            CheckStatus::Warn,
            format!(
                "{} — installer targets Debian/Ubuntu; use --dry-run elsewhere",
                os.pretty_name
            ),
        )
    };
    out.push(PreflightCheck {
        name: "Operating system".into(),
        status: os_status,
        detail: os_detail,
    });

    // Architecture
    let arch = std::env::consts::ARCH;
    let (arch_status, arch_detail) = if arch == "x86_64" {
        (CheckStatus::Ok, arch.into())
    } else {
        (CheckStatus::Warn, format!("{arch} (only x86_64 tested)"))
    };
    out.push(PreflightCheck {
        name: "Architecture".into(),
        status: arch_status,
        detail: arch_detail,
    });

    // Root / sudo
    let euid = effective_uid();
    let (root_status, root_detail) = if euid == 0 {
        (CheckStatus::Ok, "running as root".into())
    } else if have_sudo_nopasswd() {
        (
            CheckStatus::Ok,
            format!("uid={euid}, sudo NOPASSWD available"),
        )
    } else if which("sudo").is_some() {
        (
            CheckStatus::Warn,
            format!("uid={euid}, sudo present (may prompt)"),
        )
    } else {
        (
            CheckStatus::Fail,
            format!("uid={euid} and no sudo — cannot system install"),
        )
    };
    out.push(PreflightCheck {
        name: "Privileges".into(),
        status: root_status,
        detail: root_detail,
    });

    // Disk space
    out.push(check_disk_space(
        "Install dir writeable",
        install_dir,
        2 * 1024,
    ));
    out.push(check_disk_space(
        "Storage dir writeable",
        storage_dir,
        20 * 1024,
    ));

    // systemd
    let has_systemd = Path::new("/run/systemd/system").exists();
    out.push(PreflightCheck {
        name: "systemd".into(),
        status: if has_systemd {
            CheckStatus::Ok
        } else {
            CheckStatus::Warn
        },
        detail: if has_systemd {
            "present".into()
        } else {
            "not detected; services won't be managed by systemctl".into()
        },
    });

    // Network
    let net_ok = can_resolve("download.bareos.org");
    out.push(PreflightCheck {
        name: "Network (download.bareos.org)".into(),
        status: if net_ok {
            CheckStatus::Ok
        } else {
            CheckStatus::Warn
        },
        detail: if net_ok {
            "reachable".into()
        } else {
            "DNS failed; upstream-compat install needs internet".into()
        },
    });

    out
}

fn effective_uid() -> u32 {
    // /proc/self/status has Uid: r e s f — second field is effective.
    if let Ok(s) = std::fs::read_to_string("/proc/self/status") {
        for line in s.lines() {
            if let Some(rest) = line.strip_prefix("Uid:") {
                let parts: Vec<&str> = rest.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Ok(e) = parts[1].parse::<u32>() {
                        return e;
                    }
                }
            }
        }
    }
    1000
}

fn have_sudo_nopasswd() -> bool {
    use std::process::Command;
    Command::new("sudo")
        .args(["-n", "true"])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn which(cmd: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path) {
        let p = dir.join(cmd);
        if p.is_file() {
            return Some(p);
        }
    }
    None
}

fn check_disk_space(name: &str, path: &Path, needed_mib: u64) -> PreflightCheck {
    // Use sysinfo's Disks to find the mount containing `path`.
    use sysinfo::Disks;
    let disks = Disks::new_with_refreshed_list();

    let mut ancestor = path.to_path_buf();
    while !ancestor.exists() {
        if !ancestor.pop() {
            return PreflightCheck {
                name: name.into(),
                status: CheckStatus::Warn,
                detail: format!("{} does not exist yet — will be created", path.display()),
            };
        }
    }

    let mut best_len = 0usize;
    let mut best_avail: Option<u64> = None;
    for d in &disks {
        let mp = d.mount_point();
        let mp_str = mp.to_string_lossy();
        let anc_str = ancestor.to_string_lossy();
        if anc_str.starts_with(&*mp_str) && mp_str.len() > best_len {
            best_len = mp_str.len();
            best_avail = Some(d.available_space());
        }
    }

    let free_mib = best_avail.unwrap_or(0) / (1024 * 1024);
    let ok_write = writable(&ancestor);
    let status = if free_mib >= needed_mib && ok_write {
        CheckStatus::Ok
    } else if free_mib >= needed_mib {
        CheckStatus::Warn
    } else {
        CheckStatus::Fail
    };
    PreflightCheck {
        name: name.into(),
        status,
        detail: format!(
            "{}: {} MiB free (need ≥{} MiB){}",
            path.display(),
            free_mib,
            needed_mib,
            if ok_write { "" } else { ", not writable" }
        ),
    }
}

fn writable(p: &Path) -> bool {
    let probe = p.join(".nqrustbackup-installer-probe");
    match std::fs::File::create(&probe) {
        Ok(_) => {
            let _ = std::fs::remove_file(&probe);
            true
        }
        Err(_) => false,
    }
}

fn can_resolve(host: &str) -> bool {
    use std::net::ToSocketAddrs;
    (host, 443).to_socket_addrs().is_ok()
}

pub fn all_ok(checks: &[PreflightCheck]) -> bool {
    !checks.iter().any(|c| c.status == CheckStatus::Fail)
}

pub async fn run_async(
    install_dir: std::path::PathBuf,
    storage_dir: std::path::PathBuf,
) -> Result<Vec<PreflightCheck>> {
    let checks =
        tokio::task::spawn_blocking(move || run_all_checks(&install_dir, &storage_dir)).await?;
    Ok(checks)
}
