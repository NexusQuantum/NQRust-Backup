#!/usr/bin/env bash
# NQRustBackup quick installer.
#
# Downloads the latest static `nqrustbackup-installer` binary from GitHub
# Releases and runs it against this host. Works on any modern x86_64 Linux
# with curl, sudo, and bash.
#
# Usage:
#     curl -fsSL https://raw.githubusercontent.com/NexusQuantum/NQRust-Backup/master/scripts/install.sh | sudo bash
#
# Pass-through arguments (the same flags `nqrustbackup-installer install` accepts):
#     curl -fsSL <url-of-this-script> | sudo bash -s -- --profile server-only --webui-port 9100
#
# Environment variables:
#     NQRB_REPO     override repo (default: NexusQuantum/NQRust-Backup)
#     NQRB_VERSION  pin a specific tag (default: latest)
#     NQRB_NO_RUN   if set, only download the binary and print its path

set -euo pipefail

REPO="${NQRB_REPO:-NexusQuantum/NQRust-Backup}"
VERSION="${NQRB_VERSION:-latest}"
ASSET="nqrustbackup-installer-x86_64-linux-musl"

if [ "$VERSION" = "latest" ]; then
    URL="https://github.com/${REPO}/releases/latest/download/${ASSET}"
else
    URL="https://github.com/${REPO}/releases/download/${VERSION}/${ASSET}"
fi

bold()    { printf '\033[1m%s\033[0m\n' "$*"; }
muted()   { printf '\033[2m%s\033[0m\n' "$*"; }
warn()    { printf '\033[33mwarn:\033[0m %s\n' "$*" >&2; }
err()     { printf '\033[31merror:\033[0m %s\n' "$*" >&2; }

bold "NQRustBackup quick installer"
muted "  source repo: https://github.com/${REPO}"
muted "  asset:       ${ASSET}  (${VERSION})"
echo

# Sanity checks --------------------------------------------------------------

case "$(uname -m)" in
    x86_64|amd64) ;;
    *)
        err "this installer only ships an x86_64 build (got: $(uname -m)). Build from source instead:"
        err "  git clone https://github.com/${REPO}.git && cd NQRust-Backup/installer && cargo build --release"
        exit 1
        ;;
esac

if [ "$(uname -s)" != "Linux" ]; then
    err "this installer is Linux-only (got: $(uname -s))."
    exit 1
fi

if ! command -v curl >/dev/null 2>&1; then
    err "curl is required."
    exit 1
fi

# Download -------------------------------------------------------------------

TMPDIR="$(mktemp -d -t nqrb-installer-XXXXXX)"
trap 'rm -rf "$TMPDIR"' EXIT
DEST="${TMPDIR}/nqrustbackup-installer"

echo "Downloading ${URL}"
if ! curl -fsSL "$URL" -o "$DEST" 2>"${TMPDIR}/curl.err"; then
    err "failed to download from ${URL}"
    echo
    echo "This usually means:"
    echo "  1. No release has been published yet (the repo is brand new)."
    echo "  2. You are behind a network that blocks api.github.com or objects.githubusercontent.com."
    echo "  3. The version tag does not exist."
    echo
    if [ -s "${TMPDIR}/curl.err" ]; then
        echo "curl error:"
        sed 's/^/    /' "${TMPDIR}/curl.err"
    fi
    echo
    echo "To build and run from source:"
    echo "  git clone https://github.com/${REPO}.git"
    echo "  cd NQRust-Backup/installer"
    echo "  cargo build --release"
    echo "  sudo ./target/release/nqrustbackup-installer"
    exit 1
fi

if [ ! -s "$DEST" ]; then
    err "downloaded file is empty"
    exit 1
fi

chmod +x "$DEST"

# Sanity-check the binary actually runs on this host
if ! "$DEST" --version >/dev/null 2>&1; then
    err "downloaded binary does not execute on this host (libc / arch mismatch?)"
    file "$DEST" || true
    exit 1
fi

echo
bold "Downloaded: $("$DEST" --version)"
echo

if [ -n "${NQRB_NO_RUN:-}" ]; then
    INSTALL_PATH="${INSTALL_PATH:-/usr/local/bin/nqrustbackup-installer}"
    install -m 0755 "$DEST" "$INSTALL_PATH"
    echo "binary placed at: $INSTALL_PATH"
    echo "run it manually:"
    echo "  sudo $INSTALL_PATH"
    exit 0
fi

# Run -----------------------------------------------------------------------

# If invoked with no args, default to the all-in-one install. The TUI is the
# safer default when run on a real terminal, but bash piped from curl has no
# tty, so we go to the headless `install` subcommand by default.
if [ "$#" -eq 0 ]; then
    if [ -t 0 ] && [ -t 1 ]; then
        echo "Launching interactive TUI installer…"
        exec "$DEST"
    else
        echo "No TTY detected; running headless install with defaults."
        echo "  (override with flags: --source / --profile / --webui-port / --dry-run)"
        echo
        exec "$DEST" install --source upstream-compat --profile all-in-one
    fi
fi

# Pass-through: user supplied flags. Forward them to the `install` subcommand
# unless the first argument is already a subcommand.
case "${1:-}" in
    install|tui|plan|help|-h|--help|-V|--version)
        exec "$DEST" "$@"
        ;;
    *)
        exec "$DEST" install "$@"
        ;;
esac
