<p align="center">
  <img src="assets/logo_banner.png" alt="OLA Logo" width="600">
</p>

> **Open Linux Authentication: A modern, modular identity layer built for the future.**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/ij_muse/ola-core)
[![Experimental](https://img.shields.io/badge/status-experimental-orange.svg)](https://github.com/ij_muse/ola-core)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

---

## Overview

**OLA** (Open Linux Authentication) is an experimental, open-source project designed to bring next-generation login and security mechanisms to Linux systems. At its heart is **OLA Core**, a lightweight daemon that provides a clean JSON-RPC interface for client applications to integrate face authentication today, with a modular architecture ready for future biometric extensions.

This is the engine layer, not the UI. Applications like face unlock utilities, security dashboards, or login screen extensions are built on top of OLA.

## Philosophy & Name

**"OLA"** stands for **Open Linux Authentication**.

While "Olá" means "Hello" in Portuguese and Spanish—a nod to the friendly, seamless experience we aim to provide—our philosophy goes deeper. We believe Linux authentication should be:
- **Open**: Transparent, auditable, and extensible by the community.
- **Modular**: Not a monolith, but a flexible layer that supports any biometric method (Face, Fingerprint, FIDO2).
- **Native**: Built *for* Linux, respecting its philosophy of small, specialized tools working together.

We are not just copying "Windows Hello"; we are building the open, privacy-respecting foundation that Linux has always deserved.

## Why OLA?

Linux authentication is fragmented. No single layer exposes a clean, universal API for building modern biometric applications. OLA changes that by offering:

- **A simple RPC interface** for all authentication tasks.
- **A lightweight Rust core** designed for security and performance.
- **A modular structure** that welcomes plugins and additional biometric methods.
- **Clean separation** between the core engine and client applications.

This architecture allows developers to create beautiful, rich user interfaces without needing to touch low-level Linux security internals.

## Features

### Current (v1.0.0-alpha)

- **JSON-RPC Server**: Robust API over Unix domain sockets.
- **Camera Enumeration**: Reliable detection of video devices.
- **Async Rust Backend**: High-performance, non-blocking core using Tokio.
- **Dedicated Camera Worker**: Prevents resource leaks and ensures graceful shutdown.
- **Security Hardening**: Runs as unprivileged user, enforces UID allowlists, and verifies peer credentials.
- **Production-Ready**: Atomic key provisioning and systemd integration.


### Planned

- **Face Authentication**: IR camera support with liveness detection.
- **Fingerprint Support**: Integration via `libfprint`.
- **External Devices**: Authentication via YubiKeys and FIDO2 keys.
- **PIN & Passcode**: Secure local verification module.
- **PAM Integration**: Native login screen support (`pam_ola.so`).
- **Network Mode**: Optional remote authentication for cluster management.

## Use Cases

### Human-Verified AI Agent Authorization

As AI agents become part of workflows, **strict human oversight is critical**. OLA provides:

- **Presence verification** before agents execute high-risk commands
- **Biometric gating** for autonomous system authorization
- **Device authentication** to ensure agents run on trusted hardware
- **Multi-factor requirements** for agent-to-agent interactions

AI agents are powerful but not yet mature. **Humans must authenticate the agents, their systems, and their actions** using verifiable biometrics. OLA makes this possible.

### Traditional Authentication

- Face unlock for login screens (via PAM)
- Sudo replacement with biometric verification
- Screen lock/unlock
- Application-level authorization
- Multi-factor authentication



## Architecture

The goal is to isolate all authentication logic in the core, letting developers create rich apps on top.

```
┌─────────────┐         ┌──────────────┐         ┌─────────────────────┐
│   Client    │ JSON-   │  Unix Socket │ systemd │   Tokio Runtime     │
│ (Python/CLI)│ RPC     │ /run/ola/    │ socket  │   (Async I/O)       │
│             ├────────▶│   ola.sock   ├────────▶│  ┌──────────────┐   │
└─────────────┘         └──────────────┘ activ.  │  │ Connection   │   │
                                                  │  │ Handlers     │   │
                                                  │  └──────┬───────┘   │
                                                  │         │ mpsc      │
                                                  └─────────┼───────────┘
                                                            │
                                                            ▼
                                            ┌───────────────────────────┐
                                            │  Camera Worker Thread     │
                                            │   (Blocking I/O)          │
                                            │  • blocking_recv()        │
                                            │  • JoinHandle cleanup     │
                                            └──────────┬────────────────┘
                                                       │
                                                       ▼
                                              ┌────────────────┐
                                              │ /dev/video*    │
                                              └────────────────┘
```

## Installation

### Production Build

```bash
# Clone the repository
git clone https://github.com/ij_muse/ola-core.git
cd ola-core

# Install (requires root)
sudo ./install_production.sh

# Enable and start service
sudo systemctl enable --now ola.socket
sudo systemctl start ola.service
```

### Development Mode

```bash
# Build
cd core
cargo build

# Run dev server
export OLA_RUNMODE=dev
export OLA_SOCKET_PATH=/tmp/ola.sock
export RUST_LOG=info
./target/debug/ola-core &

# Test with client
TEST_OLA_SOCKET=/tmp/ola.sock python3 tests/integration_test.py
```

## API

All endpoints follow the JSON-RPC 2.0 format.

### Example: Using curl

```bash
# Ping the service
echo '{"id":1,"method":"ping","params":{}}' | \
  socat - UNIX-CONNECT:/run/ola/ola.sock
```

### Example: Using Python Client

```python
from client.ola_client import OlaClient

client = OlaClient()
print(client.ping())
# {'id': 1, 'result': {'ok': True, 'version': '0.1.0'}, 'error': None}
```

## Security

OLA Core is built with a security-first mindset:

- **No Private Keys Stored**: Credentials are managed securely on the local device.
- **Independent Codebase**: No code originates from proprietary SDKs or external AI models.
- **Least Privilege**: The daemon runs as a restricted `ola` user, not root.
- **Hardened Systemd**: Utilizes `ProtectSystem`, `PrivateTmp`, and `NoNewPrivileges`.
- **Atomic Operations**: Critical files like secret keys are created atomically to prevent race conditions.

## Roadmap

| Sprint | Status | Key Features | Priority |
|--------|--------|--------------|----------|
| **Sprint 1** | ✅ **Complete** | Async core, camera worker, systemd integration, security hardening | Shipped |
| **Sprint 2** | 🔄 In Progress | Observability, drain window, CI improvements | High |
| **Sprint 3** | 📋 Planned | Face detection (ONNX), recognition models | Medium |
| **Sprint 4** | 📋 Planned | Encrypted DB, PAM integration | Medium |

See [GOOD_FIRST_ISSUES.md](GOOD_FIRST_ISSUES.md) for ways to contribute to Sprint 2.

## Contributing

Contributions are highly welcomed! Here is how to join:

1. **Star the repo** to show support.
2. **Open issues** for ideas or problems.
3. **Submit PRs** for new modules, documentation, or performance improvements.

This project’s goal is to become a community-driven authentication layer for Linux. See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## Vision

OLA aims to make Linux beautiful, modern, and human-friendly without sacrificing freedom. Authentication shouldn’t be hard. It should be elegant, modular, and open.

**Welcome to the future of Linux security.**

---

<p align="center">
  <sub>Built with ❤️ by the Open Source Community</sub>
</p>
