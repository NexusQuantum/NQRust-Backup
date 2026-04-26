//! Configure cert-based TLS between the WebUI (PHP) and the Director.
//!
//! Background: Bareos 24+ enforces TLS at the connection layer. Even with
//! `TlsEnable = no` and `TlsRequire = no` on the Director resource, the
//! daemon still negotiates TLS-PSK on every incoming console connection
//! using the resource Password as the PSK key. The bareos-webui PHP BSock
//! library does not support TLS-PSK at all (no psk_callback in the stream
//! context), so the director resets the connection on the first non-TLS
//! byte and the webui shows "Sorry, cannot authenticate. Wrong username,
//! password or SSL/TLS handshake failed."
//!
//! Fix: stop trying to talk plain. Generate a self-signed cert at install
//! time, give it to the Director, and tell the webui to do cert-based TLS
//! (which PHP openssl supports natively) with peer verification off (the
//! cert is self-signed and only the local webui ever connects to it).
//!
//! Daemon-to-daemon TLS-PSK between DIR<->SD<->FD is left untouched —
//! those clients support PSK fine and using their own well-known shared
//! secrets is more secure than a self-signed cert anyway.

use anyhow::Result;

use crate::app::{InstallConfig, LogLevel, LogRing};
use crate::installer::executor::sudo_run_logged;

/// Provision a self-signed Director cert + wire both sides of the WebUI
/// connection to use it. Restart bareos-director.
pub async fn disable_for_console(cfg: &InstallConfig, log: &LogRing) -> Result<()> {
    log.push(
        LogLevel::Info,
        "configuring cert-based TLS between WebUI and Director",
    );

    let script = r#"set -eu

# 1. Generate a self-signed cert for the Director if not already present.
#    Used only for the WebUI<->Director TLS handshake; not for client auth.
TLS_DIR=/etc/bareos/tls
CERT="$TLS_DIR/director.crt"
KEY="$TLS_DIR/director.key"
if [ ! -f "$CERT" ] || [ ! -f "$KEY" ]; then
  mkdir -p "$TLS_DIR"
  openssl req -x509 -newkey rsa:2048 -nodes -days 3650 \
    -subj "/CN=nqrustbackup-director" \
    -keyout "$KEY" -out "$CERT" 2>/dev/null
  chown -R bareos:bareos "$TLS_DIR" 2>/dev/null || true
  chmod 700 "$TLS_DIR"
  chmod 600 "$KEY"
  chmod 644 "$CERT"
  echo "generated self-signed cert at $CERT"
else
  echo "reusing existing $CERT"
fi

# 2. Director resource: enable TLS with the self-signed cert; do NOT require
#    it (so PSK keeps working for daemon-to-daemon paths). Disable PSK
#    auto-authenticate so cert TLS is the path the WebUI takes.
DIR_CONF=/etc/bareos/bareos-dir.d/director/bareos-dir.conf
set_dir() {
  local key="$1" val="$2"
  if grep -qE "^\s*${key}\s*=" "$DIR_CONF"; then
    sed -i "s|^\(\s*${key}\s*\)=.*|\1= ${val}|" "$DIR_CONF"
  else
    sed -i "/^}\s*$/i \  ${key} = ${val}" "$DIR_CONF"
  fi
}
if [ -f "$DIR_CONF" ]; then
  set_dir "TLS Enable"        "yes"
  set_dir "TLS Require"       "no"
  set_dir "TLS Authenticate"  "no"
  set_dir "TLS Verify Peer"   "no"
  set_dir "TLS Certificate"   "\"$CERT\""
  set_dir "TLS Key"           "\"$KEY\""
fi

# 3. admin Console resource: matches Director cert TLS settings.
mkdir -p /etc/bareos/bareos-dir.d/console
cat > /etc/bareos/bareos-dir.d/console/admin.conf <<__EOF__
Console {
  Name = admin
  Password = "admin"
  Profile = "webui-admin"
  TLS Enable = yes
  TLS Require = no
  TLS Authenticate = no
  TLS Verify Peer = no
}
__EOF__
chown root:bareos /etc/bareos/bareos-dir.d/console/admin.conf 2>/dev/null || true
chmod 640 /etc/bareos/bareos-dir.d/console/admin.conf

# 4. WebUI directors.ini: tell PHP BSock to do cert TLS (which it CAN do —
#    psk it cannot). Skip peer verification (self-signed cert).
DI=/etc/bareos-webui/directors.ini
if [ -f "$DI" ]; then
  set_ini() {
    local key="$1" val="$2"
    if grep -qE "^\s*${key}\s*=" "$DI"; then
      sed -i "s|^\s*${key}\s*=.*|${key} = ${val}|" "$DI"
    else
      sed -i "/^\[localhost-dir\]/a ${key} = ${val}" "$DI"
    fi
  }
  set_ini "tls_verify_peer"      "false"
  set_ini "enable_tls_psk"       "false"
  set_ini "server_can_do_tls"    "true"
  set_ini "server_requires_tls"  "false"
  set_ini "client_can_do_tls"    "true"
  set_ini "client_requires_tls"  "false"
fi

# 5. Restart director so the cert + TLS settings on the listener take effect.
systemctl restart bareos-director
sleep 2

# 6. Probe — bconsole still works (it negotiates PSK on the same listener).
echo -e 'status director\nquit' | timeout 5 bconsole >/tmp/nqrb-tls-probe.out 2>&1 || true
if grep -q '1000 OK' /tmp/nqrb-tls-probe.out; then
  echo "cert TLS applied; bconsole reaches director (PSK still works for native clients)"
else
  echo "WARN: bconsole did not get a clean 1000 OK after TLS config:" >&2
  head -10 /tmp/nqrb-tls-probe.out >&2 || true
fi
rm -f /tmp/nqrb-tls-probe.out
"#;
    sudo_run_logged(&["sh", "-c", script], log, cfg.dry_run).await?;
    Ok(())
}
