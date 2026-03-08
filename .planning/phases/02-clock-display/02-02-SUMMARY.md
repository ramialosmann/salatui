---
phase: 02-clock-display
plan: 02
subsystem: ui
tags: [ratatui, tui, ascii-clock, crossterm, event-loop]

# Dependency graph
requires:
  - phase: 02-clock-display
    provides: "App state, digits font, next_prayer logic, Hijri formatting"
provides:
  - "Full-screen TUI with ASCII clock display (HH:MM)"
  - "Poll-based event loop with 250ms tick rate"
  - "Schedule view with next-prayer arrow and past-prayer dimming"
  - "Keyboard handling: 's' toggle view, 'q' quit"
affects: [03-notifications]

# Tech tracking
tech-stack:
  added: [ratatui 0.30.0]
  patterns: [ratatui-frame-rendering, crossterm-event-polling, double-buffered-tui]

key-files:
  created: [src/ui.rs, src/tui.rs]
  modified: [src/main.rs, Cargo.toml]

key-decisions:
  - "Countdown renders directly below clock digits, not pinned to screen bottom"
  - "hijri_date month_name_en() returns Gregorian month — mapped month numbers to English Hijri names manually"
  - "No separate crossterm dependency — ratatui 0.30.0 re-exports crossterm"
  - "ratatui::init()/restore() for terminal setup instead of manual raw mode"

patterns-established:
  - "TUI layout: date line (Length 1) + Fill content (clock+countdown centered as unit)"
  - "Direct buffer writes for ASCII digit rendering via frame.buffer_mut()"

requirements-completed: [DISP-01, DISP-02, DISP-03, DISP-04, DISP-05]

# Metrics
duration: 8min
completed: 2026-03-09
---

# Phase 2 Plan 2: TUI Rendering & Event Loop Summary

**Full-screen ratatui TUI with ASCII clock, prayer countdown, Hijri date, schedule toggle, and 250ms event loop**

## Performance

- **Duration:** 8 min
- **Started:** 2026-03-09T00:25:00Z
- **Completed:** 2026-03-09T00:33:00Z
- **Tasks:** 3
- **Files modified:** 4

## Accomplishments
- Full-screen ASCII clock display with tty-clock style HH:MM digits centered on screen
- Prayer countdown timer rendered directly below clock digits
- Hijri + Gregorian date line at top with correct Islamic month names
- Schedule view showing all 6 prayers with arrow on next, dimmed past prayers
- Poll-based event loop with 250ms tick for smooth second transitions
- Clean terminal setup/restore via ratatui::init()/restore()

## Task Commits

Each task was committed atomically:

1. **Task 1: Create ui.rs rendering and tui.rs event loop** - `ee84eae` (feat)
2. **Task 2: Wire main.rs to launch TUI with App state** - `ec44acb` (feat)
3. **Task 3: Verify TUI clock display visually** - `b891dc4` (fix — user-approved with layout + Hijri fixes)

## Files Created/Modified
- `src/ui.rs` - All rendering: draw_clock_with_countdown, draw_schedule_with_countdown, draw_date_line
- `src/tui.rs` - Terminal init/restore, poll-based event loop with 250ms tick
- `src/main.rs` - Entry point switched from print-and-exit to tui::run(app)
- `Cargo.toml` - Added ratatui 0.30.0

## Decisions Made
- Countdown placed directly under clock digits (not screen bottom) per user feedback
- Fixed hijri_date crate bug: month_name_en() returns Gregorian month, built manual Hijri month name map
- Used ratatui::init()/restore() convenience functions for terminal management
- No separate crossterm dep — ratatui 0.30.0 re-exports it

## Deviations from Plan

### Auto-fixed Issues

**1. [User Feedback] Countdown position moved from screen bottom to below clock**
- **Found during:** Task 3 (human verification)
- **Issue:** Countdown pinned to bottom of screen, user wanted it directly under clock digits
- **Fix:** Merged countdown into clock/schedule rendering functions, centered as a unit
- **Files modified:** src/ui.rs
- **Verification:** User visual approval
- **Committed in:** b891dc4

**2. [Bug Fix] Hijri month names showing Gregorian instead of Islamic**
- **Found during:** Task 3 (human verification)
- **Issue:** hijri_date crate's month_name_en() stores Gregorian month name, not Hijri
- **Fix:** Added manual mapping of month numbers 1-12 to English Hijri month names
- **Files modified:** src/app.rs
- **Verification:** User visual approval — now shows "Ramadan" correctly
- **Committed in:** b891dc4

---

**Total deviations:** 2 (1 user feedback, 1 bug fix)
**Impact on plan:** Both fixes improved correctness. No scope creep.

## Issues Encountered
- hijri_date crate's month_name_en() is misleadingly named — it actually stores the Gregorian month name from date_gr.format("%B"), not a Hijri month in English

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- TUI fully functional, ready for Phase 3 (notifications)
- App handles keyboard input ('q' quit, 's' toggle) — ready for additional keybindings
- Event loop supports tick-based updates — ready for notification trigger logic

---
*Phase: 02-clock-display*
*Completed: 2026-03-09*
