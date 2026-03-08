# tui-adhan

## What This Is

A terminal-based Islamic prayer time clock built in Rust with ratatui. Displays the current time as large ASCII art digits (tty-clock style), the Hijri date, and a countdown timer to the next prayer. Includes desktop and terminal notifications with configurable pre-alerts. Designed to run persistently in a terminal pane.

## Core Value

Always show the user exactly how long until the next prayer — at a glance, from across the room.

## Requirements

### Validated

(None yet — ship to validate)

### Active

- [ ] Big ASCII digit clock display (tty-clock style)
- [ ] Hijri (Islamic calendar) date display
- [ ] Countdown timer to next prayer (Fajr, Sunrise, Dhuhr, Asr, Maghrib, Isha)
- [ ] Local prayer time calculation using latitude/longitude
- [ ] Configurable calculation method (MWL, ISNA, Egyptian, Umm al-Qura, etc.)
- [ ] Full schedule view showing all 6 prayer times for today
- [ ] Config file at ~/.config/tui-adhan/config.toml
- [ ] CLI flag overrides for lat/lon and other settings
- [ ] Desktop notifications via notify-send/libnotify at prayer time
- [ ] Terminal bell/flash notification at prayer time
- [ ] Configurable per-prayer pre-alert notifications (X minutes before)

### Out of Scope

- Qibla direction — keeping v1 focused on time display
- Sound/adhan audio playback — adds audio dependency complexity
- API-based prayer times — local calculation preferred
- Mobile or GUI version — terminal-only

## Context

- Inspired by tty-clock's minimal, beautiful terminal aesthetic
- Six prayer events tracked: Fajr, Sunrise, Dhuhr, Asr, Maghrib, Isha
- Sunrise is included as a tracked time (not a prayer, but relevant for Fajr timing)
- Prayer calculation involves sun position math based on latitude, longitude, date, and method-specific angles
- Hijri calendar conversion is non-trivial — lunar calendar with regional variations
- Target: Linux desktop users who keep a terminal pane visible

## Constraints

- **Tech stack**: Rust with ratatui — single binary, no runtime dependencies
- **Platform**: Linux primary (notify-send for desktop notifications)
- **Config**: TOML format at ~/.config/tui-adhan/config.toml
- **Display**: Must look good in standard 80x24 terminal minimum

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Rust + ratatui | Fast rendering, single binary, great TUI ecosystem | — Pending |
| Local calculation over API | Works offline, no network dependency, more reliable | — Pending |
| TOML config with CLI overrides | Standard Rust config pattern, user-friendly | — Pending |
| Big ASCII clock as primary display | Matches tty-clock aesthetic the user wants, readable at distance | — Pending |

---
*Last updated: 2026-03-08 after initialization*
