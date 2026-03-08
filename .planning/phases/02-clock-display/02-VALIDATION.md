---
phase: 2
slug: clock-display
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-08
---

# Phase 2 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust built-in test framework (cargo test) |
| **Config file** | none (built-in, no config needed) |
| **Quick run command** | `cargo test` |
| **Full suite command** | `cargo test` |
| **Estimated runtime** | ~5 seconds |

---

## Sampling Rate

- **After every task commit:** Run `cargo test`
- **After every plan wave:** Run `cargo test`
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 5 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 02-01-01 | 01 | 1 | DISP-01 | unit | `cargo test digits` | ❌ W0 | ⬜ pending |
| 02-01-02 | 01 | 1 | DISP-02 | unit | `cargo test next_prayer` | ❌ W0 | ⬜ pending |
| 02-01-03 | 01 | 1 | DISP-03 | unit | `cargo test hijri` | ❌ W0 | ⬜ pending |
| 02-01-04 | 01 | 1 | DISP-04 | unit | `cargo test view_toggle` | ❌ W0 | ⬜ pending |
| 02-01-05 | 01 | 1 | DISP-05 | unit | `cargo test render` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `src/digits.rs` — unit tests for digit pattern correctness and rendering bounds
- [ ] `src/app.rs` — unit tests for next_prayer(), view toggle, midnight rollover
- [ ] Test for Hijri date formatting (can be in app.rs or dedicated test)
- [ ] Note: TUI rendering is hard to unit test beyond "does not panic". Visual correctness is manual.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| ASCII digits look correct visually | DISP-01 | Visual appearance cannot be automated | Run app, visually confirm digits render as large block characters |
| Layout looks balanced | DISP-01 | Aesthetic judgment | Run app at various terminal sizes, confirm layout adapts |
| Clock updates smoothly | DISP-01 | Requires observing real-time behavior | Watch for 10+ seconds, confirm no flicker |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 5s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
