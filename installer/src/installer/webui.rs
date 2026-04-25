//! WebUI bring-up (NQRustBackup UI / bareos-webui).
//!
//! On Debian/Ubuntu, the `bareos-webui` package drops an Apache/PHP app that
//! talks to the director via the standard console protocol. We need:
//! - apache2 active
//! - libapache2-mod-php installed AND the matching `a2enmod php*` enabled
//!   (without this, .php files are served as text — the symptom users hit
//!   first when they open the URL and see raw PHP)
//! - bareos-webui apache conf enabled
//! - admin console profile activated so an operator can log in
//! - a redirect from `/` to `/bareos-webui/` so the WebUI is the default
//!   landing page on the chosen port.

use anyhow::Result;

use crate::app::{InstallConfig, LogLevel, LogRing};
use crate::installer::executor::sudo_run_logged;

pub async fn setup(cfg: &InstallConfig, log: &LogRing) -> Result<()> {
    if !cfg.profile.installs_webui() {
        log.push(LogLevel::Info, "profile has no webui — skipping");
        return Ok(());
    }

    log.push(
        LogLevel::Info,
        "installing libapache2-mod-php (so .php is executed, not served as text)",
    );
    sudo_run_logged(
        &[
            "env",
            "DEBIAN_FRONTEND=noninteractive",
            "apt-get",
            "install",
            "-y",
            "libapache2-mod-php",
        ],
        log,
        cfg.dry_run,
    )
    .await?;

    log.push(LogLevel::Info, "enabling apache PHP module (a2enmod php*)");
    // Auto-detect the PHP module installed by the distro (php8.3, php8.2, etc.)
    let enable_php = r#"set -eu
mod=$(ls /etc/apache2/mods-available/ 2>/dev/null | grep -E '^php[0-9]+(\.[0-9]+)?\.load$' | head -1 | sed 's/\.load$//')
if [ -z "$mod" ]; then
  echo "no php apache module found in /etc/apache2/mods-available/" >&2
  exit 1
fi
echo "enabling apache module: $mod"
a2enmod "$mod"
"#;
    sudo_run_logged(&["sh", "-c", enable_php], log, cfg.dry_run).await?;

    log.push(LogLevel::Info, "enabling apache2 + bareos-webui conf");
    sudo_run_logged(
        &["systemctl", "enable", "--now", "apache2"],
        log,
        cfg.dry_run,
    )
    .await?;

    sudo_run_logged(&["a2enconf", "bareos-webui"], log, cfg.dry_run)
        .await
        .ok();

    // Make Apache listen on the chosen port.
    let port_setup = format!(
        r#"if ! grep -q '^Listen {}' /etc/apache2/ports.conf 2>/dev/null; then
  echo 'Listen {}' >> /etc/apache2/ports.conf
fi
"#,
        cfg.webui_port, cfg.webui_port
    );
    sudo_run_logged(&["sh", "-c", &port_setup], log, cfg.dry_run).await?;

    // Activate the admin console (package ships .example).
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

    // Redirect / → /bareos-webui/ on the WebUI port so users land where they expect.
    // We drop a dedicated VirtualHost on `cfg.webui_port` that does the redirect;
    // the `bareos-webui.conf` Alias already exposes /bareos-webui/ on every vhost.
    let redirect_conf = format!(
        r#"<VirtualHost *:{port}>
  ServerName _
  DocumentRoot /var/www/html
  RedirectMatch ^/$ /bareos-webui/
  ErrorLog ${{APACHE_LOG_DIR}}/nqrustbackup-webui-error.log
  CustomLog ${{APACHE_LOG_DIR}}/nqrustbackup-webui-access.log combined
</VirtualHost>
"#,
        port = cfg.webui_port
    );
    let install_redirect = format!(
        r#"set -eu
cat > /etc/apache2/sites-available/nqrustbackup-webui.conf <<'__EOF__'
{redirect_conf}__EOF__
a2ensite nqrustbackup-webui
"#
    );
    sudo_run_logged(&["sh", "-c", &install_redirect], log, cfg.dry_run).await?;

    // Restart so the newly enabled php module + sites are picked up cleanly.
    sudo_run_logged(&["systemctl", "restart", "apache2"], log, cfg.dry_run).await?;

    log.push(
        LogLevel::Ok,
        format!(
            "WebUI available at: http://{}:{}/  (redirects to /bareos-webui/)",
            cfg.director_address, cfg.webui_port
        ),
    );
    Ok(())
}
