# Roadmap: tui-adhan

## Overview

Build a terminal prayer time clock from the inside out: first get the math and configuration right (prayer calculation engine with full method/madhab support), then build the visual experience (big ASCII clock, countdown, Hijri date, schedule view), then bolt on notifications (desktop, terminal bell, pre-alerts). Three phases, each delivering a complete, testable capability layer.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [ ] **Phase 1: Prayer Engine** - Config loading, CLI overrides, and accurate prayer time calculation for any location/method/madhab
- [ ] **Phase 2: Clock Display** - Big ASCII clock, countdown timer, Hijri date, schedule view, and 1-second TUI event loop
- [ ] **Phase 3: Notifications** - Desktop notifications, terminal bell, and configurable per-prayer pre-alerts

## Phase Details

### Phase 1: Prayer Engine
**Goal**: User can configure their location and preferences, and the app calculates correct prayer times
**Depends on**: Nothing (first phase)
**Requirements**: CALC-01, CALC-02, CALC-03, CALC-04, CONF-01, CONF-02, CONF-03
**Success Criteria** (what must be TRUE):
  1. App reads config from ~/.config/tui-adhan/config.toml and prints all 6 prayer times for today to stdout
  2. User can override lat/lon/method via CLI flags and see different prayer times
  3. User can switch between Shafi and Hanafi Asr madhab and see the Asr time change accordingly
  4. App produces valid prayer times for high-latitude locations (e.g., Oslo in June) without crashing or showing NaN
**Plans:** 2 plans

Plans:
- [ ] 01-01-PLAN.md — Initialize Rust project, config loading/generation, CLI parsing, config/CLI merging
- [ ] 01-02-PLAN.md — Prayer calculation wrapper, enum parsing, output formatting, main.rs wiring

### Phase 2: Clock Display
**Goal**: User sees a persistent, beautiful terminal clock with prayer countdown and schedule
**Depends on**: Phase 1
**Requirements**: DISP-01, DISP-02, DISP-03, DISP-04, DISP-05
**Success Criteria** (what must be TRUE):
  1. App displays current time as large ASCII art digits that update every second
  2. App shows the name of the next prayer and a live countdown (hours:minutes:seconds) to it
  3. App displays today's Hijri date on screen
  4. User can toggle a full schedule view showing all 6 prayer times for today
  5. After Isha, the countdown correctly targets tomorrow's Fajr (midnight rollover works)
**Plans:** 1/2 plans executed

Plans:
- [ ] 02-01-PLAN.md — Core data layer: ASCII digit font, app state, next-prayer logic with midnight rollover, Hijri/countdown formatting
- [ ] 02-02-PLAN.md — TUI rendering layer: ratatui UI, event loop, main.rs wiring, visual verification

### Phase 3: Notifications
**Goal**: User never misses a prayer time, even when the terminal is not in focus
**Depends on**: Phase 2
**Requirements**: NOTF-01, NOTF-02, NOTF-03, CONF-04
**Success Criteria** (what must be TRUE):
  1. App sends a desktop notification (via notify-send) when a prayer time arrives
  2. App triggers terminal bell/flash when a prayer time arrives
  3. User can configure per-prayer pre-alert minutes in config, and receives a notification X minutes before each prayer
  4. Notifications are not duplicated (each prayer triggers at most one notification per day)
**Plans:** 1/2 plans executed

Plans:
- [ ] 03-01-PLAN.md — Notification config (NotificationConfig struct, per-prayer pre-alert overrides, default config template) and notification engine (notify-send, bell, dedup tracker, check logic)
- [ ] 03-02-PLAN.md — Wire notification engine into App state and event loop, "Prayer time!" countdown message, startup notify-send check

## Progress

**Execution Order:**
Phases execute in numeric order: 1 -> 2 -> 3

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Prayer Engine | 2/2 | Complete | 2026-03-08 |
| 2. Clock Display | 1/2 | In Progress|  |
| 3. Notifications | 1/2 | In Progress|  |
