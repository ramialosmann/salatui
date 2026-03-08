---
phase: 03-notifications
plan: 01
subsystem: notifications
tags: [notify-send, bell, toml, serde, chrono]

requires:
  - phase: 01-prayer-engine
    provides: "PrayerResult with six prayer times, AppConfig with TOML parsing"
provides:
  - "NotificationConfig with desktop/bell/pre-alert settings"
  - "PreAlertConfig with per-prayer override support"
  - "NotificationTracker for dedup across prayer notifications"
  - "check_notifications engine returning AtTime/PreAlert actions"
  - "send_desktop via notify-send, send_bell via BEL character"
affects: [03-notifications]

tech-stack:
  added: []
  patterns: ["NotificationTracker HashSet-based dedup with prayer_type keys", "PreAlertConfig per-prayer Option overrides with Sunrise defaulting to 0"]

key-files:
  created: [src/notification.rs]
  modified: [src/config.rs, src/main.rs]

key-decisions:
  - "Sunrise at-time notifications skipped by default (only fires if pre_alert.sunrise explicitly set)"
  - "Notification keys use lowercase prayer name + _at/_pre suffix for dedup tracking"
  - "At-time window is 60 seconds to avoid missed notifications on slow tick"

patterns-established:
  - "NotificationAction enum separates check from execute for testability"
  - "PreAlertConfig.get_minutes() centralizes per-prayer override logic with Sunrise special case"

requirements-completed: [CONF-04, NOTF-01, NOTF-02, NOTF-03]

duration: 2min
completed: 2026-03-09
---

# Phase 3 Plan 1: Notification Config & Engine Summary

**Notification engine with notify-send desktop alerts, terminal bell, per-prayer pre-alert overrides, and HashSet-based dedup tracking**

## Performance

- **Duration:** 2 min
- **Started:** 2026-03-08T23:07:53Z
- **Completed:** 2026-03-08T23:10:19Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments
- NotificationConfig and PreAlertConfig structs with serde defaults and per-prayer overrides
- Complete notification engine with check_notifications, execute_action, send_desktop, send_bell
- Sunrise skipped by default for both at-time and pre-alert notifications
- Default config template updated with documented [notifications] section
- 20 new tests (6 config + 10 notification + 4 tracker)

## Task Commits

Each task was committed atomically:

1. **Task 1: Add NotificationConfig to config.rs** - `e04acb8` (feat)
2. **Task 2: Create notification.rs module** - `e4b3395` (feat)

## Files Created/Modified
- `src/config.rs` - Added NotificationConfig, PreAlertConfig structs, default config template with [notifications] section
- `src/notification.rs` - New module: NotificationTracker, check_notifications, execute_action, send_desktop, send_bell, formatting helpers
- `src/main.rs` - Added mod notification declaration

## Decisions Made
- Sunrise at-time notifications skipped by default (only fires if pre_alert.sunrise explicitly set to a value)
- Notification dedup keys use lowercase prayer + _at/_pre suffix (e.g., "fajr_at", "dhuhr_pre")
- At-time detection window is 60 seconds (now >= time && now < time + 60s)

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
- Test assertion in check_notifications_at_time initially expected 2 actions but pre-alert window requires now < prayer_time, so only 1 action (at-time) fires when now == prayer_time. Fixed test assertion.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- Notification engine ready for Plan 02 to wire into TUI event loop
- NotificationTracker needs to be instantiated in App and checked on each tick
- check_notify_send_available() provided for graceful degradation if notify-send missing

---
*Phase: 03-notifications*
*Completed: 2026-03-09*
