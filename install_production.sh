#!/usr/bin/env bash
set -euo pipefail

################################################################
# install_production.sh
# - Run as root
# - Creates system user/group 'ola' if missing
# - Creates /etc/ola/secret.key (root:ola, 0600)
# - Builds release binary and installs to /usr/local/bin
# - Writes systemd unit + socket and enables socket activation
################################################################

if [ "$(id -u)" -ne 0 ]; then
  echo "Please run as root (sudo)." >&2
  exit 1
fi

echo ">>> Checking for libsodium..."
if ! ldconfig -p | grep -q libsodium; then
    echo "WARNING: libsodium not found in library cache. Ensure libsodium-devel / libsodium-dev is installed." >&2
    # We don't exit here because it might be statically linked or in a non-standard path, but we warn.
fi

# Determine repo dir relative to this script
REPO_DIR="$(dirname "$(realpath "$0")")/core"
BINARY_PATH="/usr/local/bin/ola-core"

echo ">>> Creating system user 'ola' if missing..."
if ! id -u ola >/dev/null 2>&1; then
    useradd --system --no-create-home --shell /usr/sbin/nologin ola
    echo "Created user 'ola'"
fi

echo ">>> Ensuring /etc/ola and secret key..."
mkdir -p /etc/ola
if [ ! -f /etc/ola/secret.key ]; then
    echo "Generating secret key..."
    tmp="/etc/ola/secret.key.$$"
    # Generate 32 bytes from urandom
    dd if=/dev/urandom bs=1 count=32 of="$tmp" status=none
    
    # Set permissions on temp file before moving
    chown root:ola "$tmp"
    chmod 0640 "$tmp" # Readable by root and ola group (service)
    
    # Atomic move
    mv "$tmp" /etc/ola/secret.key
    echo "Secret key created at /etc/ola/secret.key"
else
    echo "Secret key already exists."
    chown root:ola /etc/ola/secret.key || true
    chmod 0640 /etc/ola/secret.key || true
fi

echo ">>> Ensuring /etc/ola/allowlist..."
if [ ! -f /etc/ola/allowlist ]; then
    touch /etc/ola/allowlist
    echo "# Add allowed UIDs here, one per line" > /etc/ola/allowlist
    chown root:ola /etc/ola/allowlist
    chmod 0640 /etc/ola/allowlist
    echo "Allowlist created."
fi

echo ">>> Building release binary..."
if [ ! -d "$REPO_DIR" ]; then
    echo "Repository dir not found: $REPO_DIR" >&2
    exit 1
fi

cd "$REPO_DIR"
# Ensure dependencies
cargo build --release

echo ">>> Installing binary to $BINARY_PATH"
install -m 0755 target/release/ola-core "$BINARY_PATH"

echo ">>> Installing systemd socket unit (/etc/systemd/system/ola.socket)"
cat > /etc/systemd/system/ola.socket <<'EOF'
[Unit]
Description=Ola core socket
PartOf=ola.service

[Socket]
ListenStream=/run/ola/ola.sock
SocketMode=0770
SocketUser=ola
SocketGroup=ola
RuntimeDirectory=ola

[Install]
WantedBy=sockets.target
EOF

echo ">>> Installing systemd service unit (/etc/systemd/system/ola.service)"
cat > /etc/systemd/system/ola.service <<'EOF'
[Unit]
Description=Ola core service
After=network.target

[Service]
Type=simple
User=ola
Group=ola
ExecStart=/usr/local/bin/ola-core
Restart=on-failure
RuntimeDirectory=ola
PermissionsStartOnly=yes
Environment=RUST_LOG=info

# Hardening
ProtectSystem=full
PrivateTmp=yes
NoNewPrivileges=yes
ProtectHome=read-only
LimitNOFILE=65536
# CapabilityBoundingSet removed: service runs as 'ola' user, avoid granting CAP_SYS_ADMIN.

[Install]
WantedBy=multi-user.target
EOF

echo ">>> Reloading systemd and enabling socket..."
systemctl daemon-reload
systemctl enable --now ola.socket

echo ">>> Restarting service (if active)..."
systemctl restart ola.service || true

echo ">>> Done. Check status with:"
echo "  systemctl status ola.socket"
echo "  systemctl status ola.service"
echo "  journalctl -u ola.service -f"
