---
name: Bug report
about: Create a report to help us improve
title: '[BUG] '
labels: bug
assignees: ''
---

**Describe the bug**
A clear and concise description of what the bug is.

**To reproduce**
Steps to reproduce the behavior:
1. Install via `sudo ./install_production.sh`
2. Run command '...'
3. See error

**Expected behavior**
What you expected to happen.

**Logs / output**
```
# Include relevant log lines from:
sudo journalctl -u ola.service -n 50
```

**Environment**
- OS / distro / version: [e.g. Fedora 39, Ubuntu 22.04]
- Rust version: `rustc --version`
- Systemd version: `systemctl --version`
- Ola version: [e.g. 1.0.0]

**Additional context**
Add any other context about the problem here.
