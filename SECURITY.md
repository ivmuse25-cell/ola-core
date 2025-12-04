# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report security vulnerabilities by emailing:

**[ivmuse25@gmail.com]**

You should receive a response within 48 hours. If for some reason you do not, please follow up via email to ensure we received your original message.

Please include the following information in your report:

- Type of vulnerability
- Full paths of source file(s) related to the vulnerability
- Location of the affected source code (tag/branch/commit or direct URL)
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact assessment (what an attacker could do)

## Security Best Practices

When deploying OLA Core:

1. **Never commit secrets**: `/etc/ola/secret.key` should NEVER be in version control
2. **Use the allowlist**: Add only trusted UIDs to `/etc/ola/allowlist`
3. **Run as unprivileged user**: Service runs as `ola` user (not root)
4. **Keep systemd hardening**: Don't remove security directives from service unit
5. **Monitor logs**: Check `journalctl -u ola.service` for suspicious activity
6. **File permissions**: Ensure correct ownership (0640 for secrets, 0770 for socket)

## Supply Chain Security

To ensure the integrity of our dependencies:

1.  **Minimal Dependencies**: We strictly limit our dependency tree to essential, well-maintained crates (`tokio`, `nix`, `serde`).
2.  **Lockfile Tracking**: `Cargo.lock` is committed to ensure reproducible builds.
3.  **Periodic Audits**: We commit to reviewing all dependencies for vulnerabilities and updates every 6 months.
4.  **Pinned Versions**: Critical dependencies are pinned to specific minor versions to prevent breaking changes or malicious updates.

## Known Security Considerations

### Sprint 1 (Current)

- **Stubbed camera backend**: Production deployment should use real camera validation
- **Local socket only**: No network exposure by design
- **UID-based access**: Allowlist must be manually maintained

### Future Enhancements (Sprint 2+)

- Rate limiting for authentication attempts
- Structured audit logging
- Metrics and alerting for suspicious patterns

## Security Updates

Security updates will be released as patch versions (e.g., 1.0.1) and announced via:
- GitHub Security Advisories
- Release notes

## Responsible Disclosure Timeline

1. **Day 0**: Vulnerability reported
2. **Day 1-2**: Acknowledgment sent, initial assessment
3. **Day 3-7**: Investigation and fix development
4. **Day 7-14**: Testing and validation
5. **Day 14-30**: Coordinated disclosure and patch release

We appreciate your help in keeping OLA secure!
