---
phase: quick-2
plan: 1
subsystem: notifications, ui
tags: [bugfix, notification, schedule, alignment]
key-files:
  modified:
    - src/notification.rs
    - src/ui.rs
decisions:
  - Layout-based horizontal centering (Fill/Length/Fill) chosen over fixed-width padding for schedule view
metrics:
  duration: 2min
  completed: 2026-03-09
---

# Quick Task 2: Fix Pre-Alert Notification Bug and Schedule Alignment

Pre-alert notifications now trigger terminal bell when bell is enabled; schedule view uses layout-based centering to keep prayer columns aligned.

## Task Summary

| # | Task | Commit | Key Changes |
|---|------|--------|-------------|
| 1 | Fix pre-alert bell notification bug | 22abe00 | Added `if config.bell { send_bell(); }` to PreAlert arm; 2 new tests |
| 2 | Fix schedule view alignment | 883d683 | Replaced Paragraph Alignment::Center with horizontal Layout centering |

## Changes Made

### Task 1: Pre-alert bell notification bug

The `PreAlert` match arm in `execute_action()` was missing the bell check that the `AtTime` arm had. Added `if config.bell { send_bell(); }` after the desktop notification block, mirroring the `AtTime` pattern.

Added two tests:
- `test_execute_action_pre_alert_marks_tracker` - verifies PreAlert action marks tracker correctly
- `test_pre_alert_with_bell_true_calls_bell_path` - verifies code path with bell=true completes

### Task 2: Schedule view alignment

The schedule view used `Paragraph::alignment(Alignment::Center)` which independently centered each line based on its width. This caused columns to shift when the active prayer marker (2 chars) made one line wider than others.

Fix: Removed `Alignment::Center` from the Paragraph. Instead, used `Layout::horizontal([Fill(1), Length(30), Fill(1)])` to center a fixed-width block, with the paragraph rendering left-aligned within it. Title, separator, and countdown are manually padded to center within the block.

## Deviations from Plan

None - plan executed exactly as written.

## Verification

- All 45 tests pass (43 existing + 2 new)
- `cargo build` succeeds with no errors
- No regressions in any module
