---
phase: 1
slug: prayer-engine
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-08
---

# Phase 1 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust built-in test framework (cargo test) |
| **Config file** | Cargo.toml `[dev-dependencies]` section |
| **Quick run command** | `cargo test` |
| **Full suite command** | `cargo test -- --include-ignored` |
| **Estimated runtime** | ~5 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo test`
- **After every plan wave:** Run `cargo test -- --include-ignored`
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 10 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 01-01-01 | 01 | 1 | CALC-01 | unit | `cargo test prayer::tests::test_calculates_all_six_prayers` | ❌ W0 | ⬜ pending |
| 01-01-02 | 01 | 1 | CALC-02 | unit | `cargo test prayer::tests::test_different_methods` | ❌ W0 | ⬜ pending |
| 01-01-03 | 01 | 1 | CALC-03 | unit | `cargo test prayer::tests::test_madhab_affects_asr` | ❌ W0 | ⬜ pending |
| 01-01-04 | 01 | 1 | CALC-04 | unit | `cargo test prayer::tests::test_high_latitude_no_panic` | ❌ W0 | ⬜ pending |
| 01-02-01 | 02 | 1 | CONF-01 | unit | `cargo test config::tests::test_parse_config` | ❌ W0 | ⬜ pending |
| 01-02-02 | 02 | 1 | CONF-02 | unit | `cargo test config::tests::test_cli_overrides_config` | ❌ W0 | ⬜ pending |
| 01-02-03 | 02 | 1 | CONF-03 | unit | `cargo test config::tests::test_madhab_config` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `src/prayer.rs` tests module — stubs for CALC-01 through CALC-04
- [ ] `src/config.rs` tests module — stubs for CONF-01 through CONF-03
- [ ] Cargo.toml project initialization with all dependencies

*Existing infrastructure covers all phase requirements via built-in cargo test.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Output formatting (aligned columns, 12h/24h) | CONF-01 | Visual inspection | Run `cargo run -- --lat 40.7128 --lon -74.006` and verify aligned output |
| Default config file generation with comments | CONF-01 | File content inspection | Delete config, run app, verify `~/.config/tui-adhan/config.toml` has comments |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 10s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
