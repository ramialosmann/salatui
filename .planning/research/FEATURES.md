# Feature Landscape

**Domain:** Islamic prayer time TUI application (terminal clock + notifications)
**Researched:** 2026-03-08

## Table Stakes

Features users expect from any prayer time tool. Missing any of these and users will reach for a different solution.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Accurate prayer time calculation | Core purpose of the tool; wrong times = unusable | High | Use the `salah` Rust crate (based on Jean Meeus astronomical algorithms). Covers all six events: Fajr, Sunrise, Dhuhr, Asr, Maghrib, Isha |
| Multiple calculation methods | Muslims in different regions follow different authorities; without their method, times are wrong for them | Low | `salah` crate provides MWL, ISNA, Egyptian, Umm al-Qura, Karachi, Dubai, Qatar, Kuwait, Singapore, Turkey, Tehran, MoonsightingCommittee presets |
| Madhab-aware Asr calculation | Hanafi Asr is later than Shafi/standard Asr; getting this wrong alienates a large user segment | Low | `salah` supports both Shafi and Hanafi Asr shadow ratios out of the box |
| Location-based times (lat/lon) | Prayer times are meaningless without correct coordinates | Low | Config file + CLI flag overrides. No geocoding needed for v1 -- users can look up their own coordinates |
| Countdown to next prayer | The core value proposition per PROJECT.md -- "how long until next prayer, at a glance" | Medium | `salah` provides `time_remaining()` and `next()` convenience methods |
| Full daily schedule view | Users need to see all prayer times for planning, not just the next one | Low | Single-screen layout showing all six times with current/next highlighted |
| Notification at prayer time | Without alerts, user must watch the screen constantly, defeating the purpose of a persistent pane | Medium | Desktop notifications via `notify-rust` crate (wraps libnotify/D-Bus). Terminal bell as fallback |
| Configuration file | Users need to persist their location, method, and preferences | Low | TOML at `~/.config/tui-adhan/config.toml` per PROJECT.md constraints |
| CLI flag overrides | Standard UX for terminal tools; needed for quick testing and scripting | Low | `clap` crate for argument parsing |

## Differentiators

Features that set tui-adhan apart from existing CLI prayer tools (go-pray, ipraytime, cli-prayer-times, Muslimtify). Not expected, but they deliver the core aesthetic and UX vision.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Big ASCII digit clock (tty-clock style) | The defining visual identity -- readable from across the room, beautiful terminal aesthetic. No existing prayer CLI does this | Medium | Use `tui-big-text` crate with ratatui. clock-tui proves this pattern works in Rust. Need custom digit font or use figlet-style rendering |
| Hijri date display | Islamic date context matters for religious observance (fasting days, Islamic holidays). Most CLI tools skip this | High | Hijri conversion is non-trivial. Use Umm al-Qura tabular algorithm for reliability. Consider the `hijri-date` or similar crate, or implement the tabular conversion (well-documented algorithm). Accuracy concerns: tabular method is deterministic but may differ from moon-sighting by 1-2 days |
| Configurable pre-alert notifications | "10 minutes before Fajr" alerts let users prepare. Mobile apps have this; CLI tools generally do not | Medium | Per-prayer configurable offset in config. Requires tracking multiple upcoming alert times, not just the next prayer |
| Persistent terminal pane mode | Designed to run indefinitely in a tmux/terminal pane, not just print-and-exit. Most CLI tools are one-shot | Low | ratatui's event loop handles this naturally. Need clean resize handling and low CPU usage (sleep between redraws) |
| Visual prayer time highlighting | Current prayer period and next prayer visually distinct (color, bold, indicator) in the schedule view | Low | ratatui styling is straightforward. Subtle but makes the schedule scannable at a glance |
| Color theming | Terminal users care about aesthetics matching their terminal theme | Low | Support basic color configuration (foreground, accent). Can defer full theming to later |
| Responsive layout | Adapts gracefully from 80x24 minimum up to large terminals | Medium | ratatui constraint system handles this, but needs thoughtful layout design for the clock + schedule + countdown to fit at minimum size |

## Anti-Features

Features to explicitly NOT build. These are deliberate scope boundaries, not gaps.

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| Qibla direction | Adds complexity (compass bearing calculation, display challenge in TUI) without serving the core "time display" mission. Per PROJECT.md out of scope | The `salah` crate calculates Qibla, so it could be added later trivially -- but not for v1 |
| Adhan audio playback | Audio dependencies (ALSA/PulseAudio/PipeWire bindings) break the "single binary, no runtime dependencies" constraint. Cross-distro audio is painful | Desktop notifications are the alert mechanism. Users who want audio can configure `notify-send` to trigger a script |
| API-based prayer times | Network dependency makes the tool fragile and unusable offline. Local calculation is more reliable and faster | `salah` crate does all calculation locally using astronomical algorithms |
| Quran/Hadith/Dua content | Feature creep into "Islamic lifestyle app" territory. Mobile apps do this well; a TUI clock should not | Stay focused on time display. Link to resources in docs if desired |
| Prayer tracking/logging | Gamification and tracking belong in dedicated apps (Pillars, Prayerly). Adds database/state complexity | This is a clock, not a tracker |
| GUI or mobile version | Violates the terminal-only constraint. Different product entirely | Recommend existing apps (Muslim Pro, Athan) for GUI needs |
| Geocoding / auto-location | Requires network requests to convert city names to coordinates. Adds API dependency and failure modes | User provides lat/lon in config. Document how to find coordinates (e.g., from Google Maps) |
| Multi-timezone display | Adds UI complexity for a niche use case (most users care about local time only) | Show local time only. Users in different timezones reconfigure |
| Ramadan-specific features | Suhoor/Iftar times are just Fajr/Maghrib. Fasting logs, Zakat calculators are scope creep | Fajr and Maghrib times already serve Ramadan needs. Could add a "Ramadan mode" label later if there's demand |

## Feature Dependencies

```
Location config (lat/lon) --> Prayer time calculation --> All display features
                                                     --> Notification system
                                                     --> Pre-alert system

Calculation method config --> Prayer time calculation

Config file parsing --> Location config
                   --> Calculation method config
                   --> Notification preferences
                   --> Pre-alert settings
                   --> Display preferences

ratatui rendering  --> Big ASCII clock display
                   --> Full schedule view
                   --> Countdown display
                   --> Visual highlighting
                   --> Responsive layout

Prayer time calculation --> Countdown to next prayer
                       --> Notification timing
                       --> Pre-alert timing
                       --> Visual highlighting (current/next prayer)

Hijri conversion (independent) --> Hijri date display

Desktop notification system --> Prayer notifications
                            --> Pre-alert notifications
```

Key ordering insight: Prayer time calculation is the foundational dependency. Everything display-related and notification-related flows from it. The Hijri calendar is independent and can be developed in parallel.

## MVP Recommendation

Prioritize for first usable version:

1. **Prayer time calculation with configurable method** -- foundation for everything else
2. **Big ASCII clock display with countdown** -- the core visual identity and value proposition
3. **Full schedule view with highlighting** -- table stakes context for the countdown
4. **Config file with location and method** -- necessary for correct times
5. **Desktop notifications at prayer time** -- the alert mechanism users need

Defer:
- **Hijri date display**: High complexity (calendar conversion), independent feature, can ship without it. Add in phase 2.
- **Pre-alert notifications**: Medium complexity, requires per-prayer configuration UI. Add after basic notifications work.
- **Color theming**: Low priority polish. Hardcode a sensible default, make it configurable later.
- **CLI flag overrides**: Useful but not blocking. Config file is sufficient for first release.

## Sources

- [Muslim Pro](https://www.muslimpro.com/) -- leading mobile prayer app feature reference
- [Athan by IslamicFinder](https://www.islamicfinder.org/athan/) -- feature-rich mobile prayer app
- [Pillars App](https://www.thepillarsapp.com/) -- privacy-focused prayer app
- [go-pray CLI](https://github.com/0xzer0x/go-pray) -- Go-based prayer times CLI with notifications
- [salah Rust crate](https://github.com/insha/salah) -- Islamic prayer time library for Rust
- [clock-tui](https://github.com/race604/clock-tui) -- Rust TUI clock proving big-digit display pattern
- [Muslimtify](https://medium.com/@rizkirakasiwi09/integrating-muslimtify-with-waybar-prayer-times-on-your-linux-status-bar-3ceaacaad40b) -- minimalist Linux prayer notification daemon
- [Prayer calculation methods overview](https://muslimdirectoryapp.com/blog/prayer-time-calculation-methods/) -- method angle differences
- [Hijri date conversion](https://github.com/dralshehri/hijridate) -- Umm al-Qura based conversion reference
- [Best Muslim Prayer Time Apps 2025](https://muslimdirectoryapp.com/blog/best-muslim-prayer-time-apps/) -- market overview
