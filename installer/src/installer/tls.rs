//! Disable TLS on the Director-facing console paths.
//!
//! Why: Bareos packages ship with TLS-PSK enabled by default on every
//! resource. The bareos-webui PHP client cannot negotiate TLS-PSK, so users
//! get "SSL/TLS handshake failed" on login. Daemon-to-daemon TLS-PSK
//! (DIR<->SD<->FD) keeps working — those use shared secrets and have no
//! issue. We only turn off the path the WebUI traverses.
//!
//! This step is intentionally part of the always-run "render config" phase
//! (not buried in the WebUI phase) so that re-running the installer with
//! `--source configure-only` always re-applies the TLS-off settings, and
//! a fresh upstream-compat install never leaves a window where TLS is on.

use anyhow::Result;

use crate::app::{InstallConfig, LogLevel, LogRing};
use crate::installer::executor::sudo_run_logged;

/// Apply TLS-off to: Director resource, admin Console resource, and
/// /etc/bareos-webui/directors.ini. Restart bareos-director so it takes effect.
pub async fn disable_for_console(cfg: &InstallConfig, log: &LogRing) -> Result<()> {
    log.push(
        LogLevel::Info,
        "disabling TLS on Director + admin console + webui directors.ini",
    );

    let script = r#"set -eu

# 1. Director resource: TLS Enable = no, TLS Require = no
DIR_CONF=/etc/bareos/bareos-dir.d/director/bareos-dir.conf
if [ -f "$DIR_CONF" ]; then
  if grep -qE '^\s*TLS\s*Enable' "$DIR_CONF"; then
    sed -i 's/^\(\s*TLS\s*Enable\s*\)=.*/\1= no/' "$DIR_CONF"
  else
    sed -i '/^}\s*$/i \  TLS Enable = no' "$DIR_CONF"
  fi
  if grep -qE '^\s*TLS\s*Require' "$DIR_CONF"; then
    sed -i 's/^\(\s*TLS\s*Require\s*\)=.*/\1= no/' "$DIR_CONF"
  else
    sed -i '/^}\s*$/i \  TLS Require = no' "$DIR_CONF"
  fi
fi

# 2. admin Console resource: write a known-good copy with TLS off + Profile webui-admin.
mkdir -p /etc/bareos/bareos-dir.d/console
cat > /etc/bareos/bareos-dir.d/console/admin.conf <<'__EOF__'
Console {
  Name = admin
  Password = "admin"
  Profile = "webui-admin"
  TLS Enable = no
}
__EOF__
chown root:bareos /etc/bareos/bareos-dir.d/console/admin.conf 2>/dev/null || true
chmod 640 /etc/bareos/bareos-dir.d/console/admin.conf

# 3. WebUI side: directors.ini — turn off all TLS verification + PSK keys.
DI=/etc/bareos-webui/directors.ini
if [ -f "$DI" ]; then
  for KEY in tls_verify_peer enable_tls_psk tls_required tls_authenticate; do
    if grep -qE "^\s*$KEY\s*=" "$DI"; then
      sed -i "s|^\s*$KEY\s*=.*|$KEY = false|" "$DI"
    fi
  done
  # Make sure the two essential keys exist somewhere in the [localhost-dir] section.
  for KEY in tls_verify_peer enable_tls_psk; do
    grep -q "^$KEY" "$DI" || sed -i "/^\[localhost-dir\]/a $KEY = false" "$DI"
  done
fi

# 4. Restart director so the TLS settings on its listening socket are reapplied.
systemctl restart bareos-director
# 5. Probe — bconsole should connect without TLS errors.
sleep 2
echo -e 'status director\nquit' | timeout 5 bconsole >/tmp/nqrb-tls-probe.out 2>&1 || true
if grep -q '1000 OK' /tmp/nqrb-tls-probe.out; then
  echo "TLS-off applied; bconsole reaches director"
else
  echo "WARN: bconsole did not get a clean 1000 OK after TLS-off:" >&2
  head -10 /tmp/nqrb-tls-probe.out >&2 || true
fi
rm -f /tmp/nqrb-tls-probe.out
"#;
    sudo_run_logged(&["sh", "-c", script], log, cfg.dry_run).await?;
    Ok(())
}
