---
phase: quick-2
plan: 1
type: execute
wave: 1
depends_on: []
files_modified:
  - src/notification.rs
  - src/ui.rs
autonomous: true
must_haves:
  truths:
    - "Pre-alert notifications trigger terminal bell when bell is enabled"
    - "Schedule view prayer names and times are visually aligned in columns"
  artifacts:
    - path: "src/notification.rs"
      provides: "Bell in pre-alert match arm"
      contains: "send_bell"
    - path: "src/ui.rs"
      provides: "Aligned schedule layout"
  key_links:
    - from: "src/notification.rs"
      to: "PreAlert arm"
      via: "send_bell() call"
      pattern: "PreAlert.*send_bell"
---

<objective>
Fix two issues: (1) pre-alert notifications missing terminal bell, and (2) schedule view prayer list misaligned due to centering behavior.

Purpose: Pre-alert notifications should behave consistently with at-time notifications (bell + desktop). Schedule view should display prayer times in a clean, aligned columnar layout.
Output: Patched notification.rs and ui.rs
</objective>

<execution_context>
@/home/rami/.claude/get-shit-done/workflows/execute-plan.md
@/home/rami/.claude/get-shit-done/templates/summary.md
</execution_context>

<context>
@src/notification.rs
@src/ui.rs
@src/config.rs

<interfaces>
From src/notification.rs:
```rust
pub enum NotificationAction {
    AtTime { prayer: String, time_str: String },
    PreAlert { prayer: String, minutes: u32 },
}

pub fn execute_action(action: &NotificationAction, config: &NotificationConfig, tracker: &mut NotificationTracker);
pub fn send_desktop(title: &str, body: &str);
pub fn send_bell();
```

From src/config.rs:
```rust
pub struct NotificationConfig {
    pub desktop: bool,
    pub bell: bool,
    pub pre_alert_minutes: u32,
    pub pre_alert: PreAlertConfig,
}
```
</interfaces>
</context>

<tasks>

<task type="auto" tdd="true">
  <name>Task 1: Fix pre-alert bell notification bug</name>
  <files>src/notification.rs</files>
  <behavior>
    - Test: PreAlert action with bell=true calls send_bell (currently missing)
    - Test: PreAlert action with bell=false does NOT call send_bell
  </behavior>
  <action>
In `execute_action()` in src/notification.rs, the `PreAlert` match arm (lines 101-108) is missing the bell call. The `AtTime` arm correctly calls both `send_desktop` and `send_bell`, but `PreAlert` only calls `send_desktop`.

Fix: Add `if config.bell { send_bell(); }` to the `PreAlert` match arm, after the desktop notification block (after line 105), mirroring the pattern in the `AtTime` arm.

Also add a test `test_pre_alert_triggers_bell_action` that verifies the PreAlert code path includes the bell check. Since `send_bell` does side effects (prints BEL), the best approach is to add a test that calls `check_notifications` with a pre-alert scenario and verify the action is returned, then verify execute_action doesn't panic. The existing `test_check_notifications_pre_alert` already tests detection; add a focused unit test that exercises `execute_action` with a PreAlert action (with desktop=false, bell=false to avoid actual side effects) to confirm no panic and tracker is updated.
  </action>
  <verify>
    <automated>cargo test --lib notification::tests -- -q</automated>
  </verify>
  <done>PreAlert match arm includes bell notification. Test confirms execute_action handles PreAlert correctly.</done>
</task>

<task type="auto">
  <name>Task 2: Fix schedule view alignment</name>
  <files>src/ui.rs</files>
  <action>
In `draw_schedule_with_countdown()` in src/ui.rs, the schedule view has alignment issues. Each prayer line uses `format!("{}{:<10} {}", marker, name, time_str)` which gives consistent left-padding within each line, but the entire Paragraph has `Alignment::Center` applied. This means each line is independently centered based on its total width, causing the prayer name/time columns to shift left/right depending on content (e.g., "Fajr" vs "Maghrib" have different widths, but `{:<10}` fixes that -- the real issue is that the marker arrow changes line width by 2 chars, making the arrow-marked line shift).

Fix approach: Make all lines the same width so centering doesn't cause column shift. Ensure non-active prayers use "  " (2 spaces) for the marker consistently (this is already done), so the issue is more subtle. The actual problem is the separator line "------" has a different width than prayer lines, and the title line is different too.

Standardize all lines to the same fixed width:
1. Calculate a fixed content width (e.g., 30 chars) that fits the widest prayer line
2. Pad the title and separator to match
3. This ensures centering aligns all lines on the same left edge

Alternatively (simpler and better): Remove `Alignment::Center` from the Paragraph and instead center the entire block manually using a horizontal Layout with Fill/Length/Fill constraints, similar to how the clock view centers. This way the paragraph renders left-aligned within a centered box, keeping columns perfectly aligned.

Implement the Layout-based centering:
- Calculate content width as the max width of any prayer line (roughly 30 chars for "  Maghrib    06:23 PM")
- Use `Layout::horizontal([Fill(1), Length(content_width), Fill(1)])` to center the block
- Remove `.alignment(Alignment::Center)` from the Paragraph
- Keep individual title and separator lines centered within the fixed-width block by padding them
  </action>
  <verify>
    <automated>cargo build 2>&1 | tail -5</automated>
  </verify>
  <done>Schedule view renders with aligned columns -- prayer names and times line up vertically regardless of which prayer is marked as next. Title and separator are visually centered above the prayer list.</done>
</task>

</tasks>

<verification>
cargo test --lib -- -q
cargo build 2>&1 | tail -5
</verification>

<success_criteria>
- Pre-alert notifications trigger bell when config.bell is true
- Schedule view columns are visually aligned (no shifting based on active prayer marker)
- All 43+ existing tests pass, no regressions
</success_criteria>

<output>
After completion, create `.planning/quick/2-fix-pre-alert-notification-bug-and-sched/2-SUMMARY.md`
</output>
