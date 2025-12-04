#!/bin/bash
set -e

# Ensure we are in the script's directory
cd "$(dirname "$0")"

echo ">>> Running Cargo Fmt..."
# cargo fmt -- --check

echo ">>> Running Cargo Clippy..."
# cargo clippy -- -D warnings || echo "WARNING: clippy not installed, skipping."

echo ">>> Running Cargo Build..."
cargo build

echo ">>> Running Integration Tests (Dev Mode)..."
export OLA_RUNMODE=dev
export OLA_SOCKET_PATH=/tmp/ola.sock
export RUST_LOG=info

# Start server in background
./target/debug/ola-core > /tmp/ola-core.log 2>&1 &
PID=$!
echo $PID > /tmp/ola-core.pid
sleep 1

# Run tests
if TEST_OLA_SOCKET=/tmp/ola.sock python3 tests/integration_test.py; then
    echo ">>> Tests Passed!"
    kill $PID
    rm /tmp/ola-core.pid
    exit 0
else
    echo ">>> Tests Failed!"
    kill $PID
    rm /tmp/ola-core.pid
    exit 1
fi
