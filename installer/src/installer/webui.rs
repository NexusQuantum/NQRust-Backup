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

    // Write a known-good admin Console + ensure WebUI directors.ini has TLS off
    // on both sides. The packaged admin.conf.example sets `TLS Enable = no`
    // but we overwrite explicitly so a partial earlier install can't leave us
    // with TLS-PSK enabled (which the PHP webui can't negotiate → login fails
    // with "Sorry, cannot authenticate. Wrong username, password or SSL/TLS
    // handshake failed.").
    let console_activate = r#"set -eu
cat > /etc/bareos/bareos-dir.d/console/admin.conf <<'__EOF__'
Console {
  Name = admin
  Password = "admin"
  Profile = "webui-admin"
  TLS Enable = no
}
__EOF__
chown root:bareos /etc/bareos/bareos-dir.d/console/admin.conf
chmod 640 /etc/bareos/bareos-dir.d/console/admin.conf

# WebUI side: harden directors.ini so the PHP client matches.
DI=/etc/bareos-webui/directors.ini
if [ -f "$DI" ]; then
  # Comment out then re-add the keys we want, idempotently.
  sed -i 's/^\(tls_verify_peer\)\s*=.*/\1 = false/; s/^\(enable_tls_psk\)\s*=.*/\1 = false/' "$DI"
  # If the keys aren't present at all, append them under [localhost-dir] (default section name).
  grep -q '^tls_verify_peer'  "$DI" || sed -i '/^\[localhost-dir\]/a tls_verify_peer = false' "$DI"
  grep -q '^enable_tls_psk'   "$DI" || sed -i '/^\[localhost-dir\]/a enable_tls_psk = false' "$DI"
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

    // Make sure TLS is disabled on the Director resource itself, not just on
    // the admin console. The bareos-director ships with `TLS Enable = yes`
    // and `TLS Required = yes` on the Director { ... } resource, which
    // causes any console (including the WebUI) to fail with the "SSL/TLS
    // handshake failed" error. Per-console `TLS Enable = no` is overridden
    // by the Director-level setting in some Bareos versions; forcing both
    // levels off is the safe configuration for a single-host install.
    //
    // NOTE: this does NOT change daemon-to-daemon (DIR<->SD<->FD) TLS,
    // which uses fixed PSK secrets that work fine — backups still encrypt
    // in transit between the daemons.
    let dir_tls_off = r#"set -eu
DIR_CONF=/etc/bareos/bareos-dir.d/director/bareos-dir.conf
if [ -f "$DIR_CONF" ]; then
  # Replace existing TLS settings to "no", or insert before the closing brace
  # if not present.
  if grep -q '^\s*TLS\s*Enable' "$DIR_CONF"; then
    sed -i 's/^\(\s*TLS\s*Enable\s*\)=.*/\1= no/' "$DIR_CONF"
  else
    sed -i '/^}\s*$/i \  TLS Enable = no' "$DIR_CONF"
  fi
  if grep -q '^\s*TLS\s*Require' "$DIR_CONF"; then
    sed -i 's/^\(\s*TLS\s*Require\s*\)=.*/\1= no/' "$DIR_CONF"
  else
    sed -i '/^}\s*$/i \  TLS Require = no' "$DIR_CONF"
  fi
fi
"#;
    sudo_run_logged(&["sh", "-c", dir_tls_off], log, cfg.dry_run).await?;

    // Reload director so the TLS + admin console changes are live.
    sudo_run_logged(
        &[
            "sh",
            "-c",
            "systemctl restart bareos-director && sleep 1 && echo -e 'status director\nquit' | bconsole >/dev/null 2>&1 || true",
        ],
        log,
        cfg.dry_run,
    )
    .await?;

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
