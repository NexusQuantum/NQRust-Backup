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

    // Disable any global apache configs that Alias /bareos-webui or
    // /nqrustbackup-webui — we'd rather serve the WebUI as the DocumentRoot
    // of the dedicated vhost on the chosen port, so EVERY URL the rebranded
    // PHP emits resolves directly without rewriting. (Aliases + RewriteBase
    // disagreed about the prefix, which is what was causing post-login 404s.)
    let disable_old_aliases = r#"set -eu
a2disconf bareos-webui 2>/dev/null || true
a2disconf nqrustbackup-webui 2>/dev/null || true
"#;
    sudo_run_logged(&["sh", "-c", disable_old_aliases], log, cfg.dry_run).await?;

    // Make Apache listen on the chosen port.
    let port_setup = format!(
        r#"if ! grep -q '^Listen {}' /etc/apache2/ports.conf 2>/dev/null; then
  echo 'Listen {}' >> /etc/apache2/ports.conf
fi
"#,
        cfg.webui_port, cfg.webui_port
    );
    sudo_run_logged(&["sh", "-c", &port_setup], log, cfg.dry_run).await?;

    // (TLS-off for console + admin.conf + directors.ini moved to
    // installer::tls::disable_for_console, which now runs as part of the
    // always-run "render config" phase. Any install path now gets TLS off
    // by default — including --source configure-only re-runs.)

    // Drop a dedicated VirtualHost on the WebUI port whose DocumentRoot IS
    // the webui public dir. Whatever path the rebranded PHP emits — /,
    // /auth/login, /dashboard, /director/configuration, etc. — Laminas
    // routes via the standard `RewriteRule … index.php` pattern with
    // `RewriteBase /`. No prefix, no aliases, nothing to mismatch.
    let vhost_conf = format!(
        r#"<VirtualHost *:{port}>
  ServerName _
  DocumentRoot /usr/share/bareos-webui/public
  ErrorLog ${{APACHE_LOG_DIR}}/nqrustbackup-webui-error.log
  CustomLog ${{APACHE_LOG_DIR}}/nqrustbackup-webui-access.log combined

  <IfModule env_module>
    SetEnv "APPLICATION_ENV" "production"
  </IfModule>

  <Directory /usr/share/bareos-webui/public>
    Options FollowSymLinks
    AllowOverride None
    Require all granted

    <IfModule mod_rewrite.c>
      RewriteEngine on
      RewriteBase /
      RewriteCond %{{REQUEST_FILENAME}} -s [OR]
      RewriteCond %{{REQUEST_FILENAME}} -l [OR]
      RewriteCond %{{REQUEST_FILENAME}} -d
      RewriteRule ^.*$ - [NC,L]
      RewriteRule ^.*$ index.php [NC,L]
    </IfModule>
  </Directory>
</VirtualHost>
"#,
        port = cfg.webui_port
    );
    let install_vhost = format!(
        r#"set -eu
cat > /etc/apache2/sites-available/nqrustbackup-webui.conf <<'__EOF__'
{vhost_conf}__EOF__
a2ensite nqrustbackup-webui
"#
    );
    sudo_run_logged(&["sh", "-c", &install_vhost], log, cfg.dry_run).await?;

    // (Director TLS-off and bareos-director restart are handled in
    // installer::tls::disable_for_console — which already ran in the config
    // phase before this WebUI phase. We just need to bounce apache below.)

    // Restart so the newly enabled php module + sites are picked up cleanly.
    sudo_run_logged(&["systemctl", "restart", "apache2"], log, cfg.dry_run).await?;

    // Apply NQRustBackup brand overlay — downloads the rebranded webui tarball
    // from the same GitHub release the installer came from and rsync-overlays
    // it onto /usr/share/bareos-webui/. After this, the login page + theme +
    // logo are NQRustBackup, not Bareos.
    apply_brand_overlay(cfg, log).await?;

    log.push(
        LogLevel::Ok,
        format!(
            "WebUI available at: http://{}:{}/  (redirects to /bareos-webui/)",
            cfg.director_address, cfg.webui_port
        ),
    );
    Ok(())
}

async fn apply_brand_overlay(cfg: &InstallConfig, log: &LogRing) -> Result<()> {
    // Allow override via env (useful for testing PRs without re-tagging a release).
    let url = std::env::var("NQRB_WEBUI_TARBALL").unwrap_or_else(|_| {
        "https://github.com/NexusQuantum/NQRust-Backup/releases/latest/download/nqrustbackup-webui.tar.gz".to_string()
    });
    log.push(
        LogLevel::Info,
        format!("downloading NQRustBackup webui tarball: {url}"),
    );

    let script = format!(
        r#"set -eu
URL='{url}'
DEST=/usr/share/bareos-webui
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT
if ! curl -fsSL "$URL" -o "$TMP/webui.tgz"; then
  echo "WARN: could not download $URL — leaving Bareos branding in place" >&2
  exit 0
fi
mkdir -p "$TMP/extract"
tar xzf "$TMP/webui.tgz" -C "$TMP/extract"
# Overlay (no --delete: we don't want to remove deb-managed files we don't ship).
# rsync may not be installed on minimal hosts; fall back to cp -a.
if command -v rsync >/dev/null 2>&1; then
  rsync -a "$TMP/extract/" "$DEST/"
else
  cp -a "$TMP/extract/." "$DEST/"
fi
chown -R root:root "$DEST"
find "$DEST" -type d -exec chmod 755 {{}} +
find "$DEST" -type f -exec chmod 644 {{}} +
echo "applied NQRustBackup webui overlay -> $DEST"
"#
    );
    sudo_run_logged(&["sh", "-c", &script], log, cfg.dry_run).await?;

    // Restart apache so the freshly-replaced PHP files are served.
    sudo_run_logged(&["systemctl", "restart", "apache2"], log, cfg.dry_run).await?;
    Ok(())
}
