# OLA Threat Model

**Version**: 1.0.0-alpha
**Date**: 2025-12-03

## 1. System Overview

OLA (Open Linux Authentication) is a local system daemon that manages biometric authentication hardware (cameras) and exposes a JSON-RPC interface over a Unix domain socket.

### Trust Boundaries

1.  **Client <-> Daemon**: The Unix socket `/run/ola/ola.sock` is the primary boundary.
2.  **Daemon <-> Hardware**: The daemon interacts with `/dev/video*` devices.
3.  **Daemon <-> Filesystem**: The daemon reads `/etc/ola/allowlist` and `/etc/ola/secret.key`.

## 2. Assets & Risks

| Asset | Risk | Impact | Mitigation |
|-------|------|--------|------------|
| **Biometric Data** | Leakage of face data | High (Privacy violation) | No images stored persistently. Memory is cleared on process exit. Future: Encrypted templates. |
| **Secret Key** | Key theft | Critical (Forged auth tokens) | File permission `0640 root:ola`. Atomic creation. Never sent over network. |
| **Camera Feed** | Unauthorized spying | High (Privacy violation) | Daemon takes exclusive lock. LED indicator (hardware dependent). |
| **Auth Decision** | Spoofing/Bypass | Critical (System compromise) | Liveness detection (Sprint 3). Secure channel (Unix socket). |

## 3. Attack Vectors & Mitigations

### A. Malicious Client (Local User)
*   **Attack**: User tries to connect to socket to capture images or brute-force auth.
*   **Mitigation**:
    *   **Socket Permissions**: `0770 ola:ola`. Only users in `ola` group can connect.
    *   **Peer Credentials**: Daemon verifies `SO_PEERCRED` UID against `/etc/ola/allowlist`.
    *   **Rate Limiting**: (Planned Sprint 2) To prevent brute-force.

### B. Privilege Escalation
*   **Attack**: Exploiting a bug in OLA to gain root access.
*   **Mitigation**:
    *   **Least Privilege**: Daemon runs as `ola` user, NOT root.
    *   **Systemd Hardening**: `NoNewPrivileges=yes`, `ProtectSystem=full`, `PrivateTmp=yes`.
    *   **Memory Safety**: Written in Rust (memory safe).

### C. Input Fuzzing / DoS
*   **Attack**: Sending malformed JSON or massive payloads to crash the daemon.
*   **Mitigation**:
    *   **Input Validation**: Strict JSON-RPC schema validation.
    *   **Payload Limits**: 512KB max message size (enforced by codec).
    *   **Async I/O**: Tokio runtime handles concurrency; slow clients cannot block the main loop.

### D. Supply Chain
*   **Attack**: Malicious dependency introduced.
*   **Mitigation**:
    *   **Minimal Deps**: Only essential crates (`tokio`, `nix`, `serde`).
    *   **Lockfile**: `Cargo.lock` committed to version control.
    *   **Open Source**: Fully auditable codebase.

## 4. Security Posture

OLA adopts a **"Secure by Default"** philosophy. We assume the local environment may host malicious processes and rely on kernel-level enforcement (permissions, namespaces) to protect the authentication core.

## 5. Future Work

*   **Encrypted Storage**: Use `libsodium` for storing enrollment templates.
*   **TPM Integration**: Bind keys to hardware TPM if available.
*   **Audit Logging**: Tamper-evident logs for all auth attempts.

## 6. Threats Not Yet Mitigated (Roadmap)

The following threats are acknowledged but not yet fully mitigated in v1.0.0-alpha:

| Threat | Status | Planned Mitigation |
|--------|--------|--------------------|
| **Liveness / Spoofing** | ⚠️ Unmitigated | Sprint 3: IR camera liveness checks + depth sensing support. |
| **Physical Key Theft** | ⚠️ Partial | Sprint 4: TPM binding + encrypted storage at rest. |
| **Audit Trail** | ⚠️ Missing | Sprint 2: Structured, tamper-evident audit logs. |
| **Resource Exhaustion** | ⚠️ Partial | Sprint 2: Configurable rate limiting and connection throttling. |

## 7. Failure & Fallback Modes

OLA is designed to fail securely ("fail closed"):

*   **Camera Failure**: If `/dev/video*` is inaccessible, the daemon logs the error and returns `camera_error` to the client. Authentication is denied.
*   **Socket Unreachable**: If the socket is deleted or permissions change, clients cannot connect. No fallback to insecure channels.
*   **Permission Denied**: If a client UID is not in `/etc/ola/allowlist`, the connection is immediately dropped.
*   **Disk Error**: If `/etc/ola/secret.key` is unreadable, the daemon refuses to start (fatal error).

## 8. Privacy & Consent Model

OLA respects user privacy by design:

*   **No Cloud Uploads**: All biometric processing happens locally. No data ever leaves the device.
*   **Ephemeral Images**: Camera frames are processed in memory and immediately discarded. They are never written to disk (except for explicit debug/enrollment requests initiated by the user).
*   **User Consent**: Enrollment requires explicit user interaction. The daemon does not passively scan faces in the background unless a client application explicitly requests verification.
*   **Data Retention**: Future biometric templates will be stored in an encrypted format (`~/.local/share/ola/`) owned by the user, deletable at any time.
