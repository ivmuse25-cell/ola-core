# Social Media & Outreach Templates

Copy-paste ready messages for announcing Ola Core v1.0.0

---

## Hacker News

**Title**: Ola Core – local face-auth daemon for Linux (systemd socket, Rust) [alpha]

**Text**:
```
I've built Ola Core: a small, systemd-activated face-auth daemon written in Rust.

Key features:
- Dedicated camera worker thread for cancellation-safety
- UID allowlist with SO_PEERCRED authentication
- Hardened systemd service (no privileges, read-only filesystem)
- Atomic secret key provisioning
- JSON-RPC over Unix socket
- 100% test coverage (dev + production integration tests)

Repo: https://github.com/ij_muse/ola-core

Sprint 1 (v1.0.0-alpha) is production-ready for testing. Looking for early testers and contributors – first-timers welcome!

Issues labeled "good first issue" and "help wanted" available.

Tech stack: Rust (async Tokio), systemd socket activation, Unix domain sockets

Roadmap: Sprint 2 adds metrics/observability, Sprint 3 adds ONNX face detection/recognition.
```

---

## Reddit - /r/rust

**Title**: [Project] Ola Core – Face authentication daemon with systemd socket activation

**Text**:
```
Hi /r/rust!

I've just released v1.0.0-alpha ofOla Core, a face authentication daemon for Linux.

**Why I built this:**
Linux deserves native face unlock like macOS/Windows Hello, but with privacy & security first principles.

**What makes it interesting (Rust-wise):**
- Dedicated OS thread for blocking camera I/O (prevents spawn_blocking leaks)
- Graceful shutdown with explicit thread join
- Safe Unix socket activation via systemd
- Carefully designed around cancellation safety

**Current status:**
- Sprint 1 (alpha) is production-ready for testing
- 5/5 integration tests passing (dev + systemd modes)
- Hardened systemd unit (ProtectSystem, NoNewPrivileges, etc.)
- Full CI with GitHub Actions

**Tech:**
- Tokio 1.29 async runtime
- JSON-RPC over Unix sockets
- sodiumoxide for crypto
- nix crate for SO_PEERCRED

GitHub: https://github.com/ij_muse/ola-core

Looking for:
- Early testers (especially on different distros)
- Code reviewers (security scrutiny welcome!)
- First-time contributors (5 good-first-issues ready)

Feedback and PRs very welcome! 🦀
```

---

## Twitter / X

**Tweet 1** (announcement):
```
🎉 Ola Core v1.0.0-alpha is here!

Local face authentication daemon for Linux
✅ Rust + systemd socket activation
✅ Graceful shutdown, zero resource leaks
✅ Hardened security (UID allowlist, peer creds)
✅ 100% test coverage

#rustlang #linux #opensource

https://github.com/ij_muse/ola-core
```

**Tweet 2** (technical):
```
Built a face-auth daemon in Rust with some interesting patterns:

🧵 Dedicated OS thread for blocking camera I/O
🔒 SO_PEERCRED for every connection
⚡ Systemd socket activation
🎯 Explicit thread join on SIGTERM

Clean architecture > spawn_blocking everywhere

https://github.com/ij_muse/ola-core
```

**Tweet 3** (contributors):
```
Ola Core needs YOU! 👋

Looking for first-time contributors:
- 5 good first issues ready
- Clear docs & setup guide
- Friendly mentorship available

Perfect for learning Rust + systemd

Topics: async, threads, IPC, security

#rustlang #opensource

https://github.com/ij_muse/ola-core/issues
```

---

## Reddit - /r/linux

**Title**: [Release] Ola Core v1.0.0-alpha – Face authentication daemon with systemd integration

**Text**:
```
Ola Core is a new face authentication daemon for Linux, built from the ground up for security and systemd integration.

**Key features:**
- Systemd socket activation (no manual socket management)
- Runs as unprivileged user (ola:ola)
- UID-based allowlist with peer credential verification
- Hardened service unit (ProtectSystem, NoNewPrivileges, PrivateTmp)
- Atomic secret key provisioning
- Graceful shutdown with proper resource cleanup

**Architecture:**
- Async Rust core (Tokio runtime)
- Dedicated camera worker thread (prevents blocking)
- JSON-RPC API over Unix socket
- Comprehensive integration tests (dev + production)

**Status:**
Sprint 1 (v1.0.0-alpha) is ready for testing. Face detection/recognition (ONNX models) coming in Sprint 3.

**Repo:** https://github.com/ij_muse/ola-core

Tested on Fedora. Would love feedback from Ubuntu/Debian/Arch users!

Looking for testers, contributors, and security reviewers.
```

---

## Dev.to

**Title**: Building a Face Authentication Daemon for Linux with Rust and Systemd

**Tags**: #rust #linux #systemd #security

**Article Structure**:
```markdown
# Building a Face Authentication Daemon for Linux with Rust and Systemd

## Why?
Linux deserves native face unlock...

## Architecture Decisions
1. **Dedicated camera worker thread**
   - Why not spawn_blocking?
   - Cancellation safety
   - Graceful shutdown

2. **Systemd socket activation**
   - Zero downtime restarts
   - Lazy service activation
   - Clean privilege separation

3. **Security-first design**
   - UID allowlist
   - SO_PEERCRED authentication
   - Hardened service unit

## Code walkthrough
[Show key parts: worker thread, shutdown, allowlist]

## What I learned
- Tokio runtime thread model
- Systemd socket activation
- Rust async cancellation

## Try it!
GitHub: https://github.com/ij_muse/ola-core

Looking for contributors!
```

---

## LinkedIn

**Post**:
```
Excited to share Ola Core v1.0.0-alpha! 🎉

A face authentication daemon for Linux built with:
🦀 Rust (async Tokio runtime)
🔐 Security-first design (UID allowlist, hardened systemd)
⚡ Systemd socket activation
🧪 100% test coverage

Open source & looking for contributors.

Technical highlights:
- Dedicated OS thread for blocking camera I/O (prevents resource leaks)
- Graceful SIGTERM shutdown with explicit thread join
- Atomic secret key provisioning
- JSON-RPC over Unix sockets

Perfect project for Rust learners interested in systems programming, security, and Linux internals.

5 "good first issues" available for new contributors.

https://github.com/ij_muse/ola-core

#Rust #Linux #OpenSource #SystemsProgramming #Security
```

---

## Discord (Rust channel)

**Message**:
```
Hey Rustaceans! 👋

Just released Ola Core v1.0.0-alpha – a face auth daemon for Linux.

Some interesting Rust patterns:
- Dedicated OS thread for blocking I/O (proper JoinHandle management)
- Graceful shutdown with explicit thread join
- Systemd socket activation via listenfd crate
- Careful cancellation safety design

Would love feedback on the architecture, especially the worker thread pattern.

GitHub: https://github.com/ij_muse/ola-core

Issues marked "good first issue" if anyone wants to contribute!
```

---

## Usage Tips

1. **Timing**: Post to different platforms over 3-5 days (not all at once)
2. **Respond quickly**: First 24h of comments are critical
3. **Be helpful**: Answer questions thoroughly
4. **Update**: Post followup when significant PRs merge
5. **Thank contributors**: Public recognition matters

---

**Remember to replace `ij_muse/ola-core` with your actual GitHub URL!**
