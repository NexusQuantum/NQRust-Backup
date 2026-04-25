//! WebUI bring-up (NQRustBackup UI / bareos-webui).
//!
//! On Debian/Ubuntu, the `bareos-webui` package drops an Apache/PHP app that
//! talks to the director via the standard console protocol. We just need
//! apache2 active and a restricted console profile for the webui.

use anyhow::Result;

use crate::app::{InstallConfig, LogLevel, LogRing};
use crate::installer::executor::sudo_run_logged;

pub async fn setup(cfg: &InstallConfig, log: &LogRing) -> Result<()> {
    if !cfg.profile.installs_webui() {
        log.push(LogLevel::Info, "profile has no webui — skipping");
        return Ok(());
    }

    log.push(LogLevel::Info, "enabling + starting apache2 for WebUI");
    sudo_run_logged(
        &["systemctl", "enable", "--now", "apache2"],
        log,
        cfg.dry_run,
    )
    .await?;

    // The bareos-webui package drops /etc/apache2/conf-available/bareos-webui.conf
    // and enables it via postinst. Make sure it's active.
    sudo_run_logged(&["a2enconf", "bareos-webui"], log, cfg.dry_run)
        .await
        .ok();

    // Some packages default apache to :9100 via /etc/apache2/ports.conf — otherwise
    // the webui sits at http://<host>/bareos-webui/ on :80.
    let port_setup = format!(
        r#"if ! grep -q '^Listen {}' /etc/apache2/ports.conf 2>/dev/null; then
  echo 'Listen {}' >> /etc/apache2/ports.conf
fi
"#,
        cfg.webui_port, cfg.webui_port
    );
    sudo_run_logged(&["sh", "-c", &port_setup], log, cfg.dry_run).await?;

    // Ensure an admin console profile exists so an operator can log in.
    // (Package ships /etc/bareos/bareos-dir.d/console/admin.conf.example — activate if present.)
    let console_activate = r#"set -eu
src=/etc/bareos/bareos-dir.d/console/admin.conf.example
dst=/etc/bareos/bareos-dir.d/console/admin.conf
if [ -f "$src" ] && [ ! -f "$dst" ]; then
  cp "$src" "$dst"
  chown root:bareos "$dst"
  chmod 640 "$dst"
fi
"#;
    sudo_run_logged(&["sh", "-c", console_activate], log, cfg.dry_run).await?;

    sudo_run_logged(&["systemctl", "reload", "apache2"], log, cfg.dry_run).await?;

    log.push(
        LogLevel::Ok,
        format!(
            "WebUI available at: http://{}:{}/bareos-webui/",
            cfg.director_address, cfg.webui_port
        ),
    );
    Ok(())
}
