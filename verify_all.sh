#!/usr/bin/env bash
# verbose, defensive verification script for OLA
set -euo pipefail
# Turn on xtrace so you see commands as they run
export PS4='+(${BASH_SOURCE##*/}:${LINENO}) '
set -x

# Edit this if your repo is somewhere else
REPO_DIR="/home/jay/.gemini/antigravity/scratch/ola"

echo ">>> VERIFY SCRIPT START - $(date -u)"
if [ ! -d "$REPO_DIR" ]; then
  echo "ERROR: REPO_DIR not found: $REPO_DIR"
  echo "Please update REPO_DIR at top of script."
  exit 2
fi

cd "$REPO_DIR"

echo ">>> Working directory: $(pwd)"
echo ">>> Git status (short):"
git status --porcelain || true

echo ">>> Listing top-level files:"
ls -la

echo ">>> Checking .gitignore for secret.key"
if grep -q "secret.key" .gitignore; then
  echo "OK: secret.key referenced in .gitignore"
else
  echo "WARN: secret.key not found in .gitignore"
fi

echo ">>> Quick sensitive string scan (prints matches or nothing):"
grep -RIn --exclude-dir=target --exclude-dir=.git 'gemini\|antigravity\|jay' . || true

echo ">>> Rust toolchain + formatting + lint"
if ! command -v rustup >/dev/null 2>&1; then
  echo "WARN: rustup not found in PATH (install from https://rustup.rs)"; true
fi

# safe: run fmt (no fail)
cargo fmt --all || true

# run clippy but don't fail the whole script if it finds lints (print result)
if command -v cargo >/dev/null 2>&1; then
  cargo clippy --all-targets --all-features -- -D warnings || echo "clippy failed (see output above)"
else
  echo "cargo not available in PATH"
fi

echo ">>> Build debug"
cargo build

echo ">>> Run unit tests"
cargo test

echo ">>> Build release"
cargo build --release

echo ">>> cargo-audit (if installed)"
if command -v cargo-audit >/dev/null 2>&1; then
  cargo audit || echo "cargo-audit reported issues"
else
  echo "cargo-audit not installed (skipping). To install: cargo install cargo-audit"
fi

echo ">>> Quick grep for private keys or tokens (non-exhaustive)"
grep -RIn --exclude-dir=target --exclude-dir=.git -e "BEGIN RSA PRIVATE KEY" -e "PRIVATE KEY" -e "PASSWORD" -e "SECRET" -e "AKIA" -e "AIza" || true

echo ">>> Python client quick checks"
python3 -m py_compile client/ola_client.py || true
if command -v pyflakes >/dev/null 2>&1; then
  pyflakes client/ola_client.py || true
fi

echo ">>> Dev integration run"
export OLA_RUNMODE=dev
export OLA_SOCKET_PATH=/tmp/ola.sock
export RUST_LOG=info

# remove stale socket if present
if [ -e /tmp/ola.sock ]; then
  echo "Removing stale /tmp/ola.sock"
  sudo rm -f /tmp/ola.sock || true
fi

# run server in background and capture PID
./target/debug/ola-core > /tmp/ola-core-debug.log 2>&1 &
SERVER_PID=$!
echo "Started debug server PID=$SERVER_PID"
sleep 1

# run integration test - abort script on fail
python3 core/tests/integration_test.py || { echo "Integration test failed; see /tmp/ola-core-debug.log"; kill $SERVER_PID || true; exit 3; }

echo "Integration test OK; stopping debug server"
kill "$SERVER_PID" || true
wait "$SERVER_PID" 2>/dev/null || true

echo ">>> Optional systemd install test (requires sudo). Proceed? (y/N)"
read -r PROCEED
if [ "${PROCEED:-N}" = "y" ]; then
  sudo bash ./install_production.sh
  sudo systemctl daemon-reload
  sudo systemctl enable --now ola.socket
  sudo systemctl start ola.service || true
  sleep 1
  sudo systemctl status ola.socket --no-pager
  sudo systemctl status ola.service --no-pager
  if id -u ola >/dev/null 2>&1; then
    sudo -u ola TEST_OLA_SOCKET=/run/ola/ola.sock python3 core/tests/integration_test.py || echo "Prod integration failed"
  else
    echo "User 'ola' not present - skip prod-as-ola test"
  fi
fi

echo ">>> Post checks: permissions"
if [ -f /etc/ola/secret.key ]; then
  sudo stat -c '%U %G %a %n' /etc/ola/secret.key || true
else
  echo "/etc/ola/secret.key absent"
fi

if [ -e /run/ola/ola.sock ]; then
  sudo stat -c '%U %G %a %n' /run/ola/ola.sock || true
fi

echo ">>> VERIFY SCRIPT COMPLETE: ALL GREEN (if no earlier exit)"
set +x

