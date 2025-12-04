# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-12-03

### Added
- Initial production-ready release (Sprint 1)
- Async Rust core with systemd socket activation
- Camera worker thread pattern for graceful resource management
- JSON-RPC API over Unix socket (ping, list_cameras, capture_thumbnail, verify_once, status)
- Security features:
  - UID allowlist enforcement
  - Peer credentials authentication (SO_PEERCRED)
  - Hardened systemd service unit
  - Atomic secret key creation
- Production installation script with proper permissions
- Integration tests for dev and production modes
- Graceful SIGTERM shutdown with worker thread join
- Comprehensive documentation (README, SECURITY, CONTRIBUTING)

### Security
- Service runs as unprivileged `ola` user
- Secret key: 0640 root:ola permissions
- Socket: 0770 ola:ola permissions
- Systemd hardening: ProtectSystem, PrivateTmp, NoNewPrivileges
- No CAP_SYS_ADMIN capability required

### Known Limitations
- Stubbed camera backend (v4l/opencv integration planned for Sprint 3)
- No face detection/recognition yet (Sprint 3)
- No metrics/observability (Sprint 2)
- No graceful shutdown drain window (Sprint 2)

## [Unreleased]

### Planned for Sprint 2
- Graceful shutdown drain window (3s configurable)
- Metrics and observability (Prometheus)
- Automated shutdown integration tests
- Fuzz/load testing for JSON parser

### Planned for Sprint 3
- Face detection (ONNX models)
- Face recognition (MobileFaceNet/ArcFace)
- Encrypted user database (SQLite)
- PAM module integration

---

**Sprint 1 Status**: ✅ Production-Ready
