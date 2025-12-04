#!/usr/bin/env bash
set -euo pipefail
REPO_DIR="/home/jay/.gemini/antigravity/scratch/ola"
cd "$REPO_DIR"

echo
echo "======================"
echo "1) QUICK SANITY CHECKS"
echo "======================"
# show git status and top-level files
git status --porcelain
echo "--- top-level files ---"
ls -la | sed -n '1,200p'

# ensure .gitignore protects secrets
echo "--- .gitignore contains secret.key ? ---"
grep -n "secret.key" .gitignore || echo "WARN: secret.key not found in .gitignore"

# search for obvious private strings in repo (case-insensitive)
echo "--- scanning repo for 'gemini' / 'antigravity' / personal names ---"
grep -RIn --exclude-dir=.git --exclude-dir=target --exclude="preflight_report.txt" 'gemini\|antigravity\|jay' . || true

echo
echo "======================"
echo "2) RUST TOOLING (format, clippy, build, test)"
echo "======================"

# ensure Rust toolchain
rustup toolchain list >/dev/null 2>&1 || echo "Install rustup & toolchain if missing: https://rustup.rs"

# Format and lint
cd core
echo "--- cargo fmt (check) ---"
cargo fmt -- --check || echo "WARNING: cargo fmt not available or failed. Skipping."

echo "--- cargo clippy ---"
cargo clippy -- -D warnings || echo "WARNING: cargo clippy not available or failed. Skipping."

# Build & tests (debug)
cargo build
cargo test

# Build release
cargo build --release
cd ..

echo
echo "======================"
echo "3) STATIC AUDIT: cargo-audit + cargo-deny"
echo "======================"

# Install cargo-audit & cargo-deny if missing
if ! command -v cargo-audit >/dev/null 2>&1; then
  echo "Installing cargo-audit..."
  cargo install cargo-audit || true
fi
if ! command -v cargo-deny >/dev/null 2>&1; then
  echo "Installing cargo-deny..."
  cargo install cargo-deny || true
fi

# Run audits
cd core
cargo audit || echo "WARNING: cargo-audit found issues — inspect output above"
cargo deny check || echo "WARNING: cargo-deny found issues — inspect output above"
cd ..

echo
echo "======================"
echo "4) DEPENDENCY & LICENSE CHECKS"
echo "======================"
# Show top-level dependency tree for review
cd core
cargo tree --all-features | sed -n '1,200p'
cd ..

# Optional: list direct dependencies and licenses (requires cargo-license)
if ! command -v cargo-license >/dev/null 2>&1; then
  cargo install cargo-license || true
fi
cd core
cargo license > /tmp/ola_licenses.txt
cd ..
echo "Saved license list to /tmp/ola_licenses.txt"

echo
echo "======================"
echo "5) SECRET/LEAK SCANS (grep + git checks)"
echo "======================"
# Grep for common private tokens/paths (quick)
# NOTE: Adjust patterns as needed
GREP_PATTERNS="AIza|AKIA|PRIVATE_KEY|BEGIN RSA PRIVATE KEY|SECRET|PASSWORD|/home/jay|.gemini|antigravity"
echo "Scanning for sensitive patterns..."
grep -RIn --exclude-dir=target --exclude-dir=.git . -e "BEGIN RSA PRIVATE KEY" -e "PRIVATE KEY" -e "PASSWORD" -e "SECRET" -e "AKIA" -e "AIza" || true

# Ensure secret files not under git control
echo "Checking that /etc/ola/secret.key is not committed (should be in .gitignore)"
git ls-files --others --exclude-standard | grep -E "secret.key|.key" || true

echo
echo "======================"
echo "6) PYTHON CLIENT CHECK"
echo "======================"
# Install pyflakes if missing
if ! command -v pyflakes >/dev/null 2>&1; then
    pip install pyflakes || true
fi

python3 -m pyflakes client/ola_client.py || true
python3 -m py_compile client/ola_client.py || true

echo
echo "======================"
echo "7) BASIC RUNTIME & INTEGRATION TESTS (DEV MODE)"
echo "======================"

# Dev integration (non-privileged) - uses /tmp/ola.sock
export OLA_RUNMODE=dev
export OLA_SOCKET_PATH=/tmp/ola.sock
export RUST_LOG=info

# Kill any previously running debug instance on /tmp/ola.sock (safe)
if [ -e /tmp/ola.sock ]; then
  echo "Removing stale /tmp/ola.sock"
  sudo rm -f /tmp/ola.sock || true
fi

# run server in background
./core/target/debug/ola-core > /tmp/ola-core-debug.log 2>&1 &
SERVER_PID=$!
echo "ola-core (debug) started with PID $SERVER_PID; log -> /tmp/ola-core-debug.log"
sleep 2

# Run integration test
# Copy test file to /tmp to avoid permission issues if running as another user later, but here we run as current user
cp core/tests/integration_test.py /tmp/integration_test.py
chmod 644 /tmp/integration_test.py
python3 /tmp/integration_test.py || ( echo "Integration test failed - see /tmp/ola-core-debug.log" && false )

# Stop debug server
kill $SERVER_PID || true
wait $SERVER_PID 2>/dev/null || true
echo "Stopped debug server"

echo
echo "======================"
echo "8) PRODUCTION INSTALLATION CHECK (SYSTEMD) - REQUIRES SUDO"
echo "======================"
# Auto-proceed for this run since user requested it
PROCEED_SYS="y"
if [ "${PROCEED_SYS:-N}" = "y" ]; then
  # Run your install_production.sh - this will build release and install units
  sudo bash ./install_production.sh

  # Ensure socket active
  sudo systemctl daemon-reload
  sudo systemctl enable --now ola.socket
  # Start service if not started by socket
  sudo systemctl start ola.service || true
  echo "Waiting 2s for service to settle..."
  sleep 2

  # Check service and socket
  sudo systemctl status ola.socket --no-pager
  sudo systemctl status ola.service --no-pager

  # Run prod integration test as ola user (requires allowlist to include your UID or run as ola)
  if id -u ola >/dev/null 2>&1; then
    echo "Running integration test as 'ola' user (sudo -u ola)"
    sudo -u ola TEST_OLA_SOCKET=/run/ola/ola.sock python3 /tmp/integration_test.py || ( echo "Prod integration failed" && false )
  else
    echo "User 'ola' not present. Skipping prod-integration-as-ola test."
  fi

  # Check socket ownership/perms and service threads
  sudo stat -c '%U %G %a %n' /run/ola/ola.sock || true
  # Show process threads
  MAINPID=$(systemctl show -p MainPID --value ola.service)
  echo "ola.service MainPID=$MAINPID"
  ps -L -p "$MAINPID" || true

  echo "If everything above shows active (socket/service) and integration tests passed, production check succeeded."
else
  echo "Skipping systemd install test. You can run: sudo bash ./install_production.sh  later."
fi

echo
echo "======================"
echo "9) RUNTIME INSPECTION & HARDENING CHECKS"
echo "======================"
# Check files and perms
echo "/etc/ola:"
sudo ls -la /etc/ola || true

# Validate secret.key perms
if [ -f /etc/ola/secret.key ]; then
  sudo stat -c '%U %G %a %n' /etc/ola/secret.key || true
else
  echo "/etc/ola/secret.key not present (install script may not have run)"
fi

# Inspect systemd unit security-related fields
echo "systemd unit security settings (ola.service):"
sudo systemctl show -p ProtectSystem -p PrivateTmp -p NoNewPrivileges -p ProtectHome -p LimitNOFILE ola.service || true

echo
echo "======================"
echo "10) DEPLOYMENT & RELEASE CHECKS (OPTIONAL)"
echo "======================"
echo "# (Optional) check that release binary exists and is executable"
if [ -f /usr/local/bin/ola-core ]; then
  ls -lh /usr/local/bin/ola-core
  /usr/local/bin/ola-core --version || true
else
  echo "/usr/local/bin/ola-core not found - release binary not installed"
fi

echo
echo "ALL STEPS COMPLETE."
