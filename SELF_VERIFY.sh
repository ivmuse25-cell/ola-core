#!/bin/bash
set -e

echo "========================================"
echo "   OLA SELF-VERIFICATION PROTOCOL"
echo "========================================"

# Check for sudo upfront if they want to run prod tests
echo "Note: Production verification requires sudo."

echo ">>> Step 1: Building Core..."
cd core
cargo build
cd ..

echo ">>> Step 2: Running Unit Tests..."
cd core
cargo test
cd ..

echo ">>> Step 3: Running Dev Integration Tests..."
cd core
./check_all.sh
cd ..

echo ">>> Step 4: Production Verification..."
echo "This step will install OLA to /usr/local/bin and run systemd tests."
read -p "Do you want to proceed with production verification? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    sudo ./install_production.sh
    
    echo "Waiting for service to start..."
    sleep 2
    
    echo ">>> Service Status:"
    sudo systemctl status ola.service --no-pager
    
    echo ">>> Running Production Integration Test..."
    # Copy test file to /tmp so 'ola' user can read it (avoids home dir permission issues)
    cp core/tests/integration_test.py /tmp/integration_test.py
    chmod 644 /tmp/integration_test.py
    
    # Run the test as the 'ola' user to simulate real client
    sudo -u ola TEST_OLA_SOCKET=/run/ola/ola.sock python3 /tmp/integration_test.py
    
    echo ">>> Production Verification Passed!"
fi

echo "========================================"
echo "   VERIFICATION COMPLETE: ALL GREEN"
echo "========================================"
