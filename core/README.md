# Ola Core

Ola Core is the privileged face authentication daemon for the "Ola Linuxe!" project.

## Configuration

The service is configured via environment variables and configuration files.

### Environment Variables

*   `OLA_RUNMODE`: Controls socket binding behavior.
    *   `prod` (default): Expects systemd socket activation. Refuses to bind manually.
    *   `dev`: Binds manually to the socket path. Useful for testing.
*   `OLA_SOCKET_PATH`: Overrides the default socket path (`/run/ola/ola.sock`).
    *   Default: `/run/ola/ola.sock`
    *   Example: `export OLA_SOCKET_PATH=/tmp/ola.sock` (for non-root dev)
*   `RUST_LOG`: Controls logging verbosity (e.g., `info`, `debug`, `error`).

### Access Control

Access to the socket is strictly controlled.

*   **Root**: Always allowed.
*   **Service User (`ola`)**: Always allowed.
*   **Allowlist**: Explicit UIDs listed in `/etc/ola/allowlist`.
    *   Format: One UID per line. Comments start with `#`.
    *   **Production Requirement**: This file MUST exist in production mode, or the service will fail to start.

## Development

### Prerequisites

*   Rust toolchain (stable)
*   `libsodium-dev` / `libsodium-devel`
*   Python 3.9+ (for client/tests)

### Running Locally

```bash
# 1. Set Dev Mode and Socket Path
export OLA_RUNMODE=dev
export OLA_SOCKET_PATH=/tmp/ola.sock
export RUST_LOG=info

# 2. Run Server
cargo run

# 3. Run Tests (in another terminal)
export TEST_OLA_SOCKET=/tmp/ola.sock
python3 tests/integration_test.py
```

## Production Installation

Run the provided installer script as root:

```bash
sudo ./install_production.sh
```

This will:
1.  Create the `ola` system user.
2.  Install the binary to `/usr/local/bin/ola-core`.
3.  Install systemd units (`ola.service`, `ola.socket`).
4.  Initialize `/etc/ola/allowlist` and `/etc/ola/secret.key`.
