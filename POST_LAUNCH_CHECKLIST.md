# Post-Launch Checklist - Friend's Recommendations

**Based on professional review feedback**  
**Status:** Ready to implement after initial push

---

## ✅ COMPLETED (Already Done)

- [x] Clean project name and description
- [x] README + comprehensive docs
- [x] Logo & banner
- [x] Systemd/service files
- [x] Security-first defaults (SO_PEERCRED, allowlist)
- [x] CI workflow
- [x] LICENSE (MIT)
- [x] Contributor docs

---

## 🚀 PRIORITY A: GitHub Hygiene (Do First - 10 minutes)

### Repository Settings
- [ ] **Add topics** (Settings → Topics):
  - `rust`
  - `linux`
  - `authentication`
  - `biometric`
  - `systemd`
  - `security`
  - `privacy`
  - `face-recognition`
  - `ai-agents`

- [ ] **Set social preview** (Settings → Social preview):
  - Upload: `assets/logo_banner.png`

- [ ] **Enable features** (Settings → General → Features):
  - ✅ Issues (enable)
  - ✅ Discussions (enable)
  - ⬜ Projects (optional)
  - ⬜ Wiki (not needed)

- [ ] **Enable Dependabot** (Settings → Security → Dependabot):
  - Click "Enable Dependabot security updates"
  - Add `.github/dependabot.yml` (auto-generated)

### CI Verification
- [ ] Go to Actions tab
- [ ] Check latest workflow run
- [ ] Fix any failing steps (if any)

### Branch Protection
- [ ] Settings → Branches → Add rule for `main`
- [ ] Require status checks to pass (CI)
- [ ] Optional: Require pull request reviews

---

## 🤝 PRIORITY B: Community Setup (Do Second - 20 minutes)

### Create Good First Issues
- [ ] Issue #1: "Add CLI argument parsing (--version, --help)"
- [ ] Issue #2: "Replace unmaintained `users` crate with `uzers`"
- [ ] Issue #3: "Add Prometheus metrics endpoint"
- [ ] Issue #4: "Add unit tests for `check_allowlist()` function"
- [ ] Issue #5: "Improve error messages in integration tests"

**Labels to add:** `good-first-issue`, `help-wanted`, `enhancement`

### Pin Important Files
- [ ] Pin README.md
- [ ] Pin SECURITY.md  
- [ ] Pin GOOD_FIRST_ISSUES.md

### README Quick Start
- [ ] Add "Quick Start for Contributors" section at top
- [ ] Include: clone, build, test in 3 commands

---

## 💰 PRIORITY C: Funding Setup (Do Third - 15 minutes)

### Update .github/FUNDING.yml
Current file has placeholder. Update to:
```yaml
github: ivmuse25-cell  # Your actual username
# Uncomment when you set these up:
# open_collective: ola
# ko_fi: ivmuse25
# custom:
#   - https://buymeacoffee.com/ivmuse25
```

### Set Up Funding Platforms
- [ ] **GitHub Sponsors**: Apply at https://github.com/sponsors
  - May have restrictions by country
  - Best long-term option

- [ ] **Buy Me a Coffee**: https://www.buymeacoffee.com/
  - Quickest to set up (5 minutes)
  - Good for micro-donations

- [ ] **Ko-fi**: https://ko-fi.com/
  - Alternative to BMC
  - No fees on donations

- [ ] **Open Collective**: https://opencollective.com/
  - Best for transparency
  - Community governance

### Add Buy Me a Coffee Badge to README
```markdown
[![Buy Me A Coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-☕-ffdd57?style=flat-square)](https://www.buymeacoffee.com/ivmuse25)
```

---

## 📦 PRIORITY D: Release & Marketing (Do Fourth - 1 hour)

### Create v1.0.0-alpha Release
- [ ] Go to: https://github.com/ivmuse25-cell/ola-core/releases/new
- [ ] Tag: `v1.0.0-alpha`
- [ ] Title: `v1.0.0-alpha - Initial Public Release`
- [ ] Description: Use template from PUBLICATION_STRATEGY.md
- [ ] Check "Set as a pre-release"
- [ ] Publish

### Community Outreach (First 48 Hours)
- [ ] **Hacker News**: https://news.ycombinator.com/submit
  - Title: "OLA – Open Linux Authentication (Rust, privacy-first)"
  - URL: https://github.com/ivmuse25-cell/ola-core

- [ ] **Reddit r/rust**: https://reddit.com/r/rust
  - Title: "[Project] OLA - Privacy-first biometric auth daemon for Linux"
  - Use template from OUTREACH_TEMPLATES.md

- [ ] **Reddit r/linux**: https://reddit.com/r/linux
  - Focus on systemd integration and Linux-first design

- [ ] **Twitter/X Thread**:
  - Tweet 1: Announcement + banner
  - Tweet 2: AI agent security angle
  - Tweet 3: Technical highlights
  - Tweet 4: Call to action (star, contribute, sponsor)

- [ ] **LinkedIn Post**:
  - Professional angle
  - Enterprise use cases
  - AI agent authorization

### Blog Post (Optional but Recommended)
- [ ] Write on dev.to or Medium
- [ ] Topics: "Building a Secure Daemon in Rust" or "Why Linux Needs Modern Biometric Auth"

---

## 🔒 PRIORITY E: Security & Automation (Sprint 2)

### Dependabot Configuration
Create `.github/dependabot.yml`:
```yaml
version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/core"
    schedule:
      interval: "weekly"
    open-pull-requests-limit: 5
```

### Cargo Audit GitHub Action
Create `.github/workflows/security.yml`:
```yaml
name: Security Audit
on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]
  schedule:
    - cron: '0 0 * * 0'  # Weekly on Sunday

jobs:
  audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo install cargo-audit
      - run: cd core && cargo audit
```

### Branch Protection Rules
- [ ] Require passing CI before merge
- [ ] Require cargo-audit to pass
- [ ] Optional: Require 1 approving review

---

## 🎨 PRIORITY F: Polish Items (Nice to Have)

### Add Unit Tests (2-3 tests)
- [ ] `test_check_allowlist_root_allowed()`
- [ ] `test_check_allowlist_unauthorized_denied()`
- [ ] `test_remove_stale_socket()`

### Replace Deprecated Crates (Sprint 2)
- [ ] Replace `users` → `uzers`
- [ ] Replace `sodiumoxide` → Pure Rust crypto (`rust-crypto` or `ring`)

### Documentation Improvements
- [ ] Add architecture diagram (mermaid) to README
- [ ] Add "How It Works" section
- [ ] Add FAQ section

---

## 📊 Success Metrics (Track After Launch)

### Week 1 Targets
- 🌟 Stars: 50+
- 👁️ Watchers: 20+
- 🍴 Forks: 5+
- 📝 Issues opened: 10+
- 💬 Discussions: 5+

### Month 1 Targets
- 🌟 Stars: 200+
- 🤝 Contributors: 3-5
- 📦 First external package (AUR, etc.)
- 📰 Coverage in Linux news sites

---

## 🎯 IMMEDIATE ACTION PLAN (Next 2 Hours)

1. **Push current changes** (git push --force origin main)
2. **Priority A** (10 min): Add topics, enable features
3. **Priority B** (20 min): Create 5 good first issues
4. **Priority C** (15 min): Set up Buy Me a Coffee, update FUNDING.yml
5. **Priority D** (30 min): Create v1.0.0-alpha release
6. **Priority D** (45 min): Post to HN, Reddit r/rust, Twitter

**After 2 hours:** You'll have a polished, community-ready project!

---

**Next Update:** Run through checklist and mark items as done!
