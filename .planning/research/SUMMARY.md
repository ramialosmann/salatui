# Project Research Summary

**Project:** tui-adhan
**Domain:** Islamic prayer time TUI application (terminal clock + notifications)
**Researched:** 2026-03-08
**Confidence:** HIGH

## Executive Summary

tui-adhan is a persistent terminal clock that displays Islamic prayer times with big ASCII digits, a countdown to the next prayer, Hijri date, and desktop notifications. The Rust TUI ecosystem is mature enough to build this confidently: ratatui (v0.30) is the uncontested framework, the `salah` crate provides battle-tested prayer time math ported from the Adhan mobile library, and `notify-rust` handles Linux desktop notifications via D-Bus. The entire stack is synchronous -- no async runtime needed. The app fits the Elm Architecture (TEA) pattern perfectly: a 1-second tick drives state updates, and a pure view function renders the screen from that state.

The recommended approach is to build from the inside out: start with the pure-logic layer (config loading, prayer time calculation, Hijri conversion), then wire up the TEA event loop skeleton, then layer on the display (big ASCII clock, schedule grid, countdown), and finally bolt on notifications. This ordering follows natural dependency flow and lets each layer be unit-tested before the next is added. The architecture is deliberately simple -- eight modules, no threads, no async, one screen.

The primary risks are domain-specific, not technical. Prayer time calculation has subtle correctness traps: high-latitude locations where twilight formulas produce NaN, the Hanafi/Shafi Asr split that affects 30% of users, DST transition bugs, and the midnight rollover edge case for next-prayer logic. Hijri date conversion is inherently approximate and will differ from some communities by 1-2 days. Notifications can fail silently in tmux/SSH sessions. All of these are well-understood problems with documented mitigations -- the key is addressing them during implementation, not after release.

## Key Findings

### Recommended Stack

The stack is compact and dependency-light. All crates are well-maintained (except `salah`, last updated 2019, but the underlying math is stable and the crate is BSD-3 forkable). No async runtime, no HTTP client, no database. The binary should be 5-8MB.

**Core technologies:**
- **ratatui 0.30 + crossterm 0.29**: TUI rendering and terminal backend -- the standard Rust TUI stack, no real alternative
- **salah 0.7**: Prayer time calculation -- port of the battle-tested Adhan library, supports all major calculation methods and both madhabs
- **hijri_date 0.5**: Gregorian-to-Hijri conversion -- small focused crate, covers 1938-2076 CE
- **notify-rust 4.12**: Desktop notifications via D-Bus -- standard Rust notification crate, supports urgency levels and actions
- **clap 4.5 + serde + toml**: CLI parsing and TOML config -- industry standard, zero-boilerplate with derive macros
- **chrono 0.4**: Date/time handling -- already a transitive dependency of salah and hijri_date

**Key exclusions:** No tokio (sync is sufficient), no HTTP client (offline-only calculation), no ICU4X (overkill for Hijri dates), no audio playback (notifications only).

### Expected Features

**Must have (table stakes):**
- Accurate prayer time calculation with configurable method (MWL, ISNA, Egyptian, Umm al-Qura, etc.)
- Madhab-aware Asr (Hanafi vs Shafi)
- Location-based times via lat/lon config
- Countdown to next prayer
- Full daily schedule view with current/next prayer highlighted
- Desktop notifications at prayer time
- TOML configuration file at `~/.config/tui-adhan/config.toml`

**Should have (differentiators):**
- Big ASCII digit clock (tty-clock style) -- the defining visual identity, no existing prayer CLI does this
- Hijri date display
- Configurable pre-alert notifications (e.g., "10 minutes before Fajr")
- Persistent terminal pane mode (designed for tmux)
- Responsive layout (80x24 minimum to large terminals)
- Color theming

**Defer (v2+):**
- Pre-alert notifications (needs per-prayer config UI)
- Full color theming (hardcode sensible defaults first)
- CLI flag overrides (config file is sufficient for v1)
- Manual minute-offset adjustments per prayer

**Anti-features (never build):**
- Qibla direction, adhan audio, API-based times, Quran content, prayer tracking, geocoding, GUI

### Architecture Approach

The Elm Architecture (TEA) is the right pattern: a `Message` enum (Tick, Quit, ToggleSchedule, Resize) drives an `update()` function that mutates the `App` model, and a `view()` function renders the model to the terminal. The 1-second tick is the heartbeat. Prayer times are calculated once per day and cached. Notifications are deduplicated via a `HashSet<(Prayer, NaiveDate)>` sent-flags pattern.

**Major components (8 modules):**
1. **config.rs** -- TOML loading, CLI merge, defaults with `#[serde(default)]`
2. **prayer.rs** -- Thin wrapper around `salah`, exposes app-specific types, isolates crate API
3. **hijri.rs** -- Thin wrapper around `hijri_date`, converts today's date
4. **app.rs** -- Model struct holding all state (time, prayers, countdown, notification flags)
5. **event.rs** -- Keyboard + tick polling via crossterm, emits `Message` enum
6. **ui.rs** -- Full screen layout using ratatui (clock zone, info zone, schedule zone)
7. **clock.rs** -- Big ASCII digit rendering widget
8. **notification.rs** -- Desktop notifications via notify-rust + terminal bell fallback

### Critical Pitfalls

1. **High-latitude prayer time failure** -- Fajr/Isha formulas produce NaN above ~48.5N in summer. The `salah` crate handles this, but verify with Reykjavik/Oslo coordinates in June. Must be correct from day one.
2. **Asr madhab mismatch** -- Hanafi Asr differs by 30-60 minutes from Shafi. Support both via config from the start; omitting Hanafi alienates ~30% of users.
3. **Midnight rollover in next-prayer logic** -- After Isha, "next prayer" is tomorrow's Fajr. Must always calculate tomorrow's times too. Breaks every night if missed.
4. **Desktop notifications failing silently** -- D-Bus unavailable in tmux/SSH/Wayland edge cases. Always pair with terminal bell fallback. Show failure status in TUI.
5. **DST transition bugs** -- Prayer times jump by one hour if timezone offset is cached. Use timezone names (not offsets), recalculate daily, compare against UTC instants.

## Implications for Roadmap

Based on research, suggested phase structure:

### Phase 1: Foundation (Config + Prayer Calculation)
**Rationale:** Everything depends on correct prayer times from correct configuration. This is the foundational dependency -- no display or notification feature works without it.
**Delivers:** Config file loading with defaults, prayer time calculation for any location/method/madhab, basic CLI entry point.
**Addresses:** Location config, calculation method config, madhab-aware Asr -- all table stakes.
**Avoids:** Fixed twilight angle assumption (Pitfall 1), Asr mismatch (Pitfall 3), DST bugs (Pitfall 6), config crash (Pitfall 10).

### Phase 2: Core TUI Loop + Clock Display
**Rationale:** With prayer calculation working, wire up the TEA event loop and the primary visual -- the big ASCII clock with countdown. This is the core value proposition and can be validated visually.
**Delivers:** Ratatui event loop with 1-second tick, big ASCII digit clock, countdown to next prayer, basic schedule view with highlighting.
**Addresses:** Big ASCII clock (key differentiator), countdown to next prayer (table stakes), full schedule view, persistent pane mode.
**Avoids:** CPU waste from fast render loop (Pitfall 8), small terminal garbling (Pitfall 9), midnight rollover (Pitfall 7), Sunrise-as-prayer confusion (Pitfall 12).

### Phase 3: Hijri Date Display
**Rationale:** Independent of the prayer/display pipeline, but adds important context. Medium complexity due to calendar conversion subtleties. Best done as a focused phase after the core display is stable.
**Delivers:** Hijri date displayed below the clock, using Umm al-Qura algorithm via `hijri_date` crate.
**Addresses:** Hijri date display (differentiator).
**Avoids:** Off-by-one Hijri date (Pitfall 5).

### Phase 4: Notifications
**Rationale:** Side-effect layer bolted onto an already-working clock. Needs fallback-first design. Separate phase because notification reliability requires careful error handling and testing across environments.
**Delivers:** Desktop notifications at prayer time via notify-rust, terminal bell fallback, notification deduplication, in-TUI failure reporting.
**Addresses:** Notification at prayer time (table stakes).
**Avoids:** Silent notification failure (Pitfall 4), Umm al-Qura Isha Ramadan adjustment (Pitfall 11).

### Phase 5: Polish + Resilience
**Rationale:** With all features working, harden edge cases and improve UX.
**Delivers:** Responsive layout with graceful degradation, default config generation on first run, pre-alert notifications, color configuration.
**Addresses:** Responsive layout, pre-alert notifications, color theming (differentiators).

### Phase Ordering Rationale

- Phases 1-2 follow the architecture's natural dependency chain: config feeds prayer calculation feeds display. You cannot test display without data, and you cannot generate data without config.
- Hijri (Phase 3) is architecturally independent but deferred because it is not table stakes. It can be developed in parallel with Phase 2 if desired.
- Notifications (Phase 4) are deliberately last among core features because they are a side-effect on top of a working clock. Getting the clock right first means notification timing can be validated against the display.
- This ordering matches the architecture document's suggested build order exactly, which increases confidence it is correct.

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 1 (Prayer Calculation):** The `salah` crate API needs hands-on validation. Verify high-latitude handling, Umm al-Qura Ramadan Isha adjustment, and `next_prayer()` behavior across midnight. Test against aladhan.com reference times.
- **Phase 3 (Hijri Date):** Validate `hijri_date` crate accuracy against Umm al-Qura official calendar. Decide whether Hijri date flips at midnight or Maghrib.

Phases with standard patterns (skip research-phase):
- **Phase 2 (TUI Loop + Clock):** ratatui TEA pattern is extremely well-documented with official templates and tutorials. Big-text rendering is proven by `tui-big-text` and `clock-tui`.
- **Phase 4 (Notifications):** `notify-rust` is straightforward. The design pattern (check flag, send, set flag) is simple.
- **Phase 5 (Polish):** Standard ratatui layout and config work.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | All crates are well-documented with clear APIs. Only `salah` (unmaintained since 2019) is a minor concern, but the math is stable and a fallback plan exists. |
| Features | HIGH | Feature landscape is clear from comparable apps (Muslim Pro, go-pray, clock-tui). Table stakes vs differentiators well-defined. |
| Architecture | HIGH | TEA pattern for ratatui is officially recommended and well-documented. Component boundaries are clean. |
| Pitfalls | HIGH | Domain pitfalls are extensively documented in Islamic prayer time calculation literature. All have known mitigations. |

**Overall confidence:** HIGH

### Gaps to Address

- **salah crate midnight behavior:** Verify whether `salah`'s `next_prayer()` correctly handles the post-Isha-to-tomorrow's-Fajr transition. If not, implement the tomorrow-calculation workaround in `prayer.rs`.
- **salah crate Umm al-Qura Ramadan adjustment:** The crate docs say to add +30 min to Isha during Ramadan manually. Determine if this needs Hijri date detection (circular dependency with Phase 3) or if it can use approximate Gregorian Ramadan dates.
- **hijri_date accuracy:** No independent validation found. Compare against islamicfinder.org for a sample of dates before relying on it.
- **tui-big-text compatibility:** Confirm `tui-big-text` works with ratatui 0.30. If not, the big digit rendering is a custom widget (~50 lines of digit lookup tables).

## Sources

### Primary (HIGH confidence)
- [ratatui docs and TEA pattern](https://ratatui.rs/concepts/application-patterns/the-elm-architecture/) -- architecture pattern
- [ratatui GitHub + templates](https://github.com/ratatui/ratatui) -- framework reference
- [salah crate docs](https://docs.rs/salah/latest/salah/) -- prayer calculation API
- [crossterm docs](https://docs.rs/crossterm/0.29.0/crossterm/) -- terminal backend
- [notify-rust docs](https://docs.rs/notify-rust/latest/notify_rust/) -- notification API
- [clap docs](https://docs.rs/clap/4.5.60/clap/) -- CLI parsing
- [PrayTimes.org calculation docs](https://praytimes.org/docs/calculation) -- prayer time formulas

### Secondary (MEDIUM confidence)
- [salah GitHub](https://github.com/insha/salah) -- last updated 2019, math is stable but API may have undocumented edge cases
- [hijri_date docs](https://docs.rs/hijri_date/0.5.1/hijri_date/) -- limited documentation, needs validation
- [AlAdhan calculation methods](https://aladhan.com/calculation-methods) -- method angle reference
- [clock-tui](https://github.com/race604/clock-tui) -- proves big-digit TUI pattern in Rust

### Tertiary (LOW confidence)
- [Fiqh Council angle discussion](https://fiqhcouncil.org/fifteen-or-eighteen-degrees-calculating-prayer-fasting-times-in-islam/) -- angle accuracy debate, informational only

---
*Research completed: 2026-03-08*
*Ready for roadmap: yes*
