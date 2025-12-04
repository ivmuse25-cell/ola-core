# Good First Issues - Welcome Contributors! 👋

These issues are perfect for first-time contributors to Ola Core. Each includes step-by-step guidance.

---

## Issue 1: Add Prometheus Metrics Scaffold

**Title**: Add basic Prometheus metrics endpoint

**Labels**: `good first issue`, `enhancement`, `observability`

**Description**:

Add a `/metrics` endpoint (or an exported collector) that tracks:
- `total_requests` (counter)
- `request_failures` (counter)
- `request_duration_seconds` (histogram)

Make it optional behind a feature flag to avoid adding HTTP server in Sprint 1.

**Steps**:
1. Add `prometheus` crate to `Cargo.toml` with optional feature
2. Create `src/metrics.rs` module
3. Add counters using `lazy_static!`
4. Increment counters in request handler (main.rs)
5. Add simple text exporter that prints to stdout (for now)
6. Document in README

**Estimated Time**: 2-3 hours

**Mentor**: Available for questions

---

## Issue 2: Improve README - Add Architecture Diagram

**Title**: Add architecture diagram to README

**Labels**: `good first issue`, `documentation`

**Description**:

Create a simple ASCII or small PNG diagram showing:
- Tokio runtime
- Camera worker thread
- Request flow
- Shutdown sequence

Add it to the "Architecture" section of README.

**Steps**:
1. Use tool like asciiflow.com or draw.io
2. Show main thread, worker thread, and message passing
3. Keep it simple and readable
4. Add to README.md after "Quick Start" section

**Estimated Time**: 1-2 hours

**Example**:
```
┌─────────────┐      ┌──────────────┐
│  Client     │─────▶│  Unix Socket │
└─────────────┘      └──────┬───────┘
                            │
                     ┌──────▼───────┐
                     │  Tokio RT    │
                     │  (async)     │
                     └──────┬───────┘
                            │ mpsc::channel
                     ┌──────▼───────┐
                     │  Camera      │
                     │  Worker      │
                     │  Thread      │
                     └──────────────┘
```

---

## Issue 3: Add CI Job for Integration Tests

**Title**: Add GitHub Actions step to run integration_test.py

**Labels**: `good first issue`, `testing`, `ci`

**Description**:

Modify `.github/workflows/ci.yml` to:
- Build debug binary
- Start server in background (dev mode)
- Run `integration_test.py`
- Capture logs on failure
- Tear down cleanly

Make sure it runs without requiring root.

**Steps**:
1. Edit `.github/workflows/ci.yml`
2. Add step after "Build (debug)"
3. Set environment variables (OLA_RUNMODE=dev, etc.)
4. Start server: `./target/debug/ola-core &`
5. Run tests: `python3 tests/integration_test.py`
6. Kill server after tests
7. Test locally with Act: `act -j build-and-test`

**Estimated Time**: 1-2 hours

**Files to modify**: `.github/workflows/ci.yml`

---

## Issue 4: Add Configurable Drain Window

**Title**: Make shutdown drain window configurable via environment variable

**Labels**: `good first issue`, `enhancement`

**Description**:

The current graceful shutdown has a 3-second drain window. Make this configurable via `OLA_DRAIN_SECS` environment variable.

**Steps**:
1. In `main.rs`, read `OLA_DRAIN_SECS` env var
2. Parse to u64, default to 3
3. Use in shutdown: `Duration::from_secs(drain_secs)`
4. Document in README under "Configuration"
5. Add test case

**Estimated Time**: 1 hour

**Related**: Sprint 2 high-priority item

---

## Issue 5: Add Unit Tests for `check_allowlist`

**Title**: Add unit tests for allowlist validation logic

**Labels**: `good first issue`, `testing`

**Description**:

The `check_allowlist()` function in `main.rs` currently has no unit tests. Add comprehensive tests covering:
- Valid UID in allowlist
- Invalid UID not in allowlist
- Empty allowlist
- Comments in allowlist
- Malformed allowlist

**Steps**:
1. Create `#[cfg(test)]` module in `main.rs`
2. Write test helper to create temp allowlist file
3. Add test cases (at least 5)
4. Run with `cargo test check_allowlist`

**Estimated Time**: 1-2 hours

**Example**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_check_allowlist_valid_uid() {
        // Create temp file with "1000"
        // Assert check_allowlist(1000) == Ok(true)
    }
}
```

---

## How to Claim an Issue

1. Comment "I'd like to work on this!" on the issue
2. Wait for maintainer assignment (usually < 24 hours)
3. Fork the repo and create a branch
4. Make your changes
5. Submit a PR with "Fixes #issue-number" in description

## Need Help?

- Ask questions in the issue comments
- Join discussions tab
- Check [CONTRIBUTING.md](CONTRIBUTING.md) for setup guide

**Welcome aboard!** 🚀
