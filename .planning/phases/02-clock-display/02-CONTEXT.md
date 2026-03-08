# Phase 2: Clock Display - Context

**Gathered:** 2026-03-08
**Status:** Ready for planning

<domain>
## Phase Boundary

Persistent TUI showing a big ASCII clock, countdown timer to next prayer, Hijri date, and toggleable full schedule view — all updating every second. No notifications, no sound, no config changes beyond display preferences.

</domain>

<decisions>
## Implementation Decisions

### Clock visual style
- tty-clock replica: same blocky ASCII digit style, same minimalist feel
- No border box around the clock — bare digits, borderless mode
- HH:MM only (no seconds in the clock digits)
- Monochrome — white/default terminal color only, no color
- Clock centered in the terminal

### Screen layout
- Vertical stack, all centered: dates → clock → countdown
- Hijri + Gregorian date line at top: "14 Ramadan 1447 AH · 8 Mar 2026"
- Big ASCII clock digits in the middle
- Countdown below clock: "Maghrib in 1:23:45" (H:MM:SS format, ticks every second)

### Schedule toggle
- 's' key toggles schedule view on/off
- 'q' key quits the app
- Schedule view replaces the clock (full screen takeover), not alongside it
- Hijri/Gregorian date line and countdown timer remain visible in schedule view
- Next prayer marked with ▶ arrow marker
- Past prayers shown in dim/gray text
- No keybinding help hints on screen

### Hijri date
- Format: "14 Ramadan 1447 AH" — English month names, AH suffix
- Gregorian date shown alongside: "14 Ramadan 1447 AH · 8 Mar 2026" (single line, dot separator)
- Algorithmic Hijri conversion is fine — no manual offset config needed

### Claude's Discretion
- ASCII digit font implementation (exact pixel pattern for each digit)
- Hijri conversion library choice
- ratatui widget structure and layout percentages
- Event loop implementation (crossterm vs other backends)
- Exact spacing between elements
- Error handling for terminal size too small

</decisions>

<specifics>
## Specific Ideas

- tty-clock is the primary aesthetic inspiration — the app should feel like tty-clock with Islamic prayer info added
- Schedule view mockup: title "Today's Prayer Times", horizontal rule, aligned prayer name + time columns

</specifics>

<code_context>
## Existing Code Insights

### Reusable Assets
- `prayer.rs`: `PrayerResult` struct with all 6 prayer times as `chrono::DateTime<Local>` — direct data source for countdown and schedule
- `prayer.rs`: `calculate_prayers(lat, lon, method, madhab)` — call once per day, cache result
- `config.rs`: `AppConfig` with `DisplayConfig` — extend with new display settings if needed
- `config.rs`: `load_or_create_config()` + `merge_config_with_cli()` — reuse for TUI startup

### Established Patterns
- clap for CLI parsing, serde + toml for config, anyhow for errors
- chrono for time handling (Local timezone)
- Time format config: `display.time_format` already supports "24h" / "12h"

### Integration Points
- `main.rs` currently prints and exits — needs to switch to TUI event loop mode
- Prayer calculation called once at startup (and recalculated at midnight for next day)
- Config struct shared — may need new fields if display preferences are added

</code_context>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 02-clock-display*
*Context gathered: 2026-03-08*
