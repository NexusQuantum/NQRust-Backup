//! Post-install verification checks.

use anyhow::Result;
use std::process::Command;

use crate::app::{CheckStatus, InstallConfig, LogLevel, LogRing, PreflightCheck};

pub async fn run(cfg: &InstallConfig, log: &LogRing) -> Result<Vec<PreflightCheck>> {
    let cfg = cfg.clone();
    let log = log.clone();
    tokio::task::spawn_blocking(move || run_blocking(&cfg, &log)).await?
}

fn run_blocking(cfg: &InstallConfig, log: &LogRing) -> Result<Vec<PreflightCheck>> {
    let mut out = Vec::new();

    if cfg.profile.installs_director() {
        out.push(unit_active_check("bareos-director"));
    }
    if cfg.profile.installs_storage() {
        out.push(unit_active_check("bareos-storage"));
    }
    if cfg.profile.installs_filedaemon() {
        out.push(unit_active_check("bareos-filedaemon"));
    }

    if cfg.profile.installs_catalog() {
        out.push(catalog_check());
    }

    if cfg.profile.installs_director() {
        out.push(bconsole_check());
    }

    if cfg.profile.installs_webui() {
        // Root should redirect to /bareos-webui/ (after v0.1.3 install)
        let root = format!("http://{}:{}/", cfg.director_address, cfg.webui_port);
        out.push(http_check("WebUI root (expect 301/302)", &root));
        let app = format!(
            "http://{}:{}/bareos-webui/",
            cfg.director_address, cfg.webui_port
        );
        out.push(http_check("WebUI app (expect 200)", &app));
    }

    for c in &out {
        log.push(
            if c.status == CheckStatus::Ok {
                LogLevel::Ok
            } else {
                LogLevel::Warn
            },
            format!("verify: {} → {}", c.name, c.detail),
        );
    }

    Ok(out)
}

fn unit_active_check(unit: &str) -> PreflightCheck {
    let out = Command::new("systemctl").args(["is-active", unit]).output();
    let (status, detail) = match out {
        Ok(o) => {
            let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
            if s == "active" {
                (CheckStatus::Ok, "active".into())
            } else {
                (CheckStatus::Fail, s)
            }
        }
        Err(e) => (CheckStatus::Fail, format!("systemctl failed: {e}")),
    };
    PreflightCheck {
        name: format!("{unit} service"),
        status,
        detail,
    }
}

fn catalog_check() -> PreflightCheck {
    let out = Command::new("sudo")
        .args([
            "-u",
            "bareos",
            "psql",
            "-d",
            "bareos",
            "-tAc",
            "SELECT count(*) FROM version",
        ])
        .output();
    match out {
        Ok(o) if o.status.success() => PreflightCheck {
            name: "Catalog reachable".into(),
            status: CheckStatus::Ok,
            detail: format!(
                "version rows: {}",
                String::from_utf8_lossy(&o.stdout).trim()
            ),
        },
        Ok(o) => PreflightCheck {
            name: "Catalog reachable".into(),
            status: CheckStatus::Fail,
            detail: String::from_utf8_lossy(&o.stderr).trim().to_string(),
        },
        Err(e) => PreflightCheck {
            name: "Catalog reachable".into(),
            status: CheckStatus::Fail,
            detail: e.to_string(),
        },
    }
}

fn bconsole_check() -> PreflightCheck {
    let out = Command::new("sh")
        .args([
            "-c",
            "echo -e 'status director\nquit' | timeout 5 bconsole 2>&1 | head -n 20",
        ])
        .output();
    match out {
        Ok(o) if o.status.success() => {
            let s = String::from_utf8_lossy(&o.stdout);
            if s.contains("OK:") || s.contains("Director") {
                PreflightCheck {
                    name: "bconsole → director".into(),
                    status: CheckStatus::Ok,
                    detail: "status director replied".into(),
                }
            } else {
                PreflightCheck {
                    name: "bconsole → director".into(),
                    status: CheckStatus::Warn,
                    detail: s.lines().last().unwrap_or("no reply").to_string(),
                }
            }
        }
        Ok(o) => PreflightCheck {
            name: "bconsole → director".into(),
            status: CheckStatus::Warn,
            detail: String::from_utf8_lossy(&o.stderr).trim().to_string(),
        },
        Err(e) => PreflightCheck {
            name: "bconsole → director".into(),
            status: CheckStatus::Warn,
            detail: e.to_string(),
        },
    }
}

fn http_check(name: &str, url: &str) -> PreflightCheck {
    let out = Command::new("curl")
        .args([
            "-sS",
            "-o",
            "/dev/null",
            "-w",
            "%{http_code}",
            "--max-time",
            "5",
            url,
        ])
        .output();
    match out {
        Ok(o) if o.status.success() => {
            let code = String::from_utf8_lossy(&o.stdout).trim().to_string();
            let ok = matches!(code.as_str(), "200" | "301" | "302");
            PreflightCheck {
                name: name.into(),
                status: if ok {
                    CheckStatus::Ok
                } else {
                    CheckStatus::Warn
                },
                detail: format!("HTTP {code} @ {url}"),
            }
        }
        Ok(o) => PreflightCheck {
            name: name.into(),
            status: CheckStatus::Warn,
            detail: String::from_utf8_lossy(&o.stderr).trim().to_string(),
        },
        Err(e) => PreflightCheck {
            name: name.into(),
            status: CheckStatus::Warn,
            detail: e.to_string(),
        },
    }
}
