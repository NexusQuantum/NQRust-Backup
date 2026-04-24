# NQRust Backup Linux Runtime

This folder documents the Linux path for the whitelabel NQRust Backup stack.

## Local Linux Smoke Stack

From the repository root on a Linux host:

```bash
bash _local/build_linux.sh
bash _local/start_all_linux.sh
```

Access:

```text
WebUI:    http://127.0.0.1:9104/
REST API: http://127.0.0.1:8000/docs
Console:  bash _local/nqrustbackup_console_linux.sh
```

Default local WebUI login:

```text
Director: localhost-dir
Username: admin
Password: local-password
```

Stop everything:

```bash
bash _local/stop_all_linux.sh
```

## Prerequisites

The local stack expects a Linux build toolchain, PostgreSQL client/server tools, PHP CLI, Python venv/pip, and common development headers. On Debian/Ubuntu, the baseline is:

```bash
sudo apt install build-essential cmake ninja-build pkg-config postgresql postgresql-client php-cli php-gettext python3 python3-venv python3-pip libssl-dev libpq-dev libreadline-dev zlib1g-dev libjansson-dev libcap-dev libacl1-dev liblzo2-dev liblmdb-dev libpam0g-dev libsystemd-dev libsqlite3-dev
```

For a Linux desktop tray monitor build, install Qt development packages and build with:

```bash
TRAYMONITOR=ON bash _local/build_linux.sh
```

On a headless Linux server, tray monitor is intentionally skipped; WebUI is the production user interface.

## Production Direction

For production, prefer installing into system paths and running under systemd plus a real web server or PHP-FPM setup. The local scripts are intentionally self-contained for validation and staging.

Recommended production layout:

```text
/usr/sbin/nqrustbackup-dir
/usr/sbin/nqrustbackup-fd
/usr/sbin/nqrustbackup-sd
/etc/nqrustbackup
/etc/nqrustbackup-webui
/var/lib/nqrustbackup
/var/log/nqrustbackup
```

Build options for a system install:

```bash
cmake -S . -B _local/build-linux-system -G Ninja \
  -DCMAKE_BUILD_TYPE=Release \
  -DCMAKE_INSTALL_PREFIX=/usr \
  -DCMAKE_INSTALL_SYSCONFDIR=/etc \
  -DCMAKE_INSTALL_LOCALSTATEDIR=/var \
  -DENABLE_WEBUI=ON \
  -DENABLE_SYSTEMTESTS=OFF \
  -Dsystemd=ON
cmake --build _local/build-linux-system --target install --parallel "$(nproc)"
```

Run the local smoke stack first before converting it to package/systemd deployment.
