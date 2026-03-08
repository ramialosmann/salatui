# Phase 3: Notifications - Context

**Gathered:** 2026-03-09
**Status:** Ready for planning

<domain>
## Phase Boundary

Desktop notifications (via notify-send), terminal bell, and configurable per-prayer pre-alerts so the user never misses a prayer time even when the terminal is not in focus. No sound/adhan playback, no new display features beyond a brief "Prayer time!" countdown message.

</domain>

<decisions>
## Implementation Decisions

### Notification content
- Prayer name as notification title (not app name)
- Body text: "Prayer time: 6:23 PM" (respects 12h/24h config)
- Include mosque emoji (🕌) in body: "🕌 Maghrib"
- Pre-alert format distinct from at-time: "Maghrib in 15 minutes" (no emoji, countdown style)
- At-time format: title "Maghrib", body "🕌 Prayer time: 6:23 PM"

### Pre-alert config
- Global default + per-prayer overrides in TOML
- Config structure: `[notifications]` section with `desktop = true`, `bell = true`, `pre_alert_minutes = 15`
- Per-prayer overrides: `[notifications.pre_alert]` with `fajr = 20`, `sunrise = 0`, etc.
- Setting a prayer's pre-alert to 0 disables the pre-alert only — at-time notification still fires
- Separate toggles for desktop notifications vs terminal bell (not one master toggle)
- Pre-alerts trigger desktop notification only — bell reserved for actual prayer time

### TUI visual feedback
- No TUI visual changes (flash, color, highlight) when prayer arrives — keeps monochrome minimal aesthetic
- Countdown shows "Maghrib — Prayer time!" for 1 minute when prayer time arrives, then switches to next prayer countdown
- Message replaces the countdown line in the same position

### Dedup & edge cases
- No missed notifications — only notify for future prayer times at app startup
- In-memory tracking only (HashSet of notified prayers) — no persistence to file
- If notify-send not available: warn once at startup to stderr, disable desktop notifications, bell and TUI still work
- Skip Sunrise notifications by default — not a prayer. User can explicitly enable via per-prayer pre-alert config

### Claude's Discretion
- Notification check timing precision within the 250ms tick loop
- How to detect notify-send availability (which/command -v vs try-and-fail)
- HashSet structure for tracking notified prayers
- Terminal bell escape sequence choice (\x07 vs alternative)
- Error handling for failed notify-send invocations

</decisions>

<specifics>
## Specific Ideas

- Pre-alerts are gentler (desktop only), at-time is the strong signal (desktop + bell)
- "Prayer time!" message format: "Maghrib — Prayer time!" with em dash, displayed for exactly 1 minute

</specifics>

<code_context>
## Existing Code Insights

### Reusable Assets
- `app.rs`: `next_prayer()` returns `(&str, DateTime<Local>)` — use to detect when prayer time arrives
- `app.rs`: `prayer_list()` returns all 6 prayer `(name, time)` tuples — iterate for notification scheduling
- `app.rs`: `tick()` already handles midnight rollover recalculation — notification state should reset here too
- `config.rs`: `AppConfig` with nested section pattern — extend with `NotificationConfig` section
- `config.rs`: `generate_default_config()` raw string — add `[notifications]` section

### Established Patterns
- clap for CLI, serde + toml for config, anyhow for errors
- chrono `DateTime<Local>` for all time comparisons
- 250ms tick rate in event loop — notification checks run each tick
- Config merge pattern: TOML defaults + CLI overrides

### Integration Points
- `tui.rs`: event loop `run_loop()` — add notification check after `app.tick()`
- `app.rs`: `App` struct — add notification state (notified set, pre-alerted set, prayer-time message timer)
- `config.rs`: `AppConfig` — add `NotificationConfig` with `desktop`, `bell`, `pre_alert_minutes`, per-prayer overrides
- `main.rs`: startup — add notify-send availability check before entering TUI

</code_context>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 03-notifications*
*Context gathered: 2026-03-09*
