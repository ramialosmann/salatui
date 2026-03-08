# Technology Stack

**Project:** tui-adhan
**Researched:** 2026-03-08

## Recommended Stack

### Core TUI Framework
| Technology | Version | Purpose | Why | Confidence |
|------------|---------|---------|-----|------------|
| [ratatui](https://ratatui.rs/) | 0.30.0 | TUI rendering, widgets, layout | The standard Rust TUI framework. Actively maintained, modular since 0.30, massive widget ecosystem. No real competitor in Rust TUI space. | HIGH |
| [crossterm](https://docs.rs/crossterm/0.29.0) | 0.29.0 | Terminal backend for ratatui | Default backend for ratatui on Linux. Handles raw mode, events, cursor. Pure Rust, no C dependencies. | HIGH |

### Prayer Time Calculation
| Technology | Version | Purpose | Why | Confidence |
|------------|---------|---------|-----|------------|
| [salah](https://github.com/insha/salah) | 0.7.6 | Islamic prayer time calculation | Port of the well-tested Batoul Apps Adhan library. Supports MWL, Egyptian, Karachi, Umm al-Qura, Dubai, Qatar, Kuwait, Singapore, Turkey, Tehran, North America (ISNA), and custom methods. Includes Hanafi/Shafi madhab for Asr. Astronomical calculations from Jean Meeus's book. | MEDIUM |

**Note on salah:** Last commit was January 2019. This is a concern for long-term maintenance, but prayer time algorithms are mathematically stable -- the underlying astronomy does not change. The library is a faithful port of Adhan (used in production iOS/Android apps). The math is correct and battle-tested. If bugs are found, forking is straightforward since it is BSD-3 licensed.

**Alternative considered:** `praytime-rs` (1.0.1) -- a PrayTimes.org port with similar method support. Newer but less proven lineage than salah's Adhan heritage. Either would work; salah's API is more ergonomic with its builder pattern (`PrayerSchedule`).

**Fallback plan:** If salah proves problematic, implement calculation directly from the PrayTimes.org algorithms. The math is well-documented at [praytimes.org/docs/calculation](https://praytimes.org/docs/calculation) and involves ~200 lines of trigonometry.

### Hijri Calendar
| Technology | Version | Purpose | Why | Confidence |
|------------|---------|---------|-----|------------|
| [hijri_date](https://docs.rs/hijri_date/0.5.1) | 0.5.1 | Gregorian-to-Hijri date conversion | Purpose-built for Islamic (lunar) Hijri calendar. Covers years 1356-1500 AH (1938-2076 CE). Supports formatting (`%Y %M %D`), date comparison, and arithmetic. Uses chrono internally. MIT licensed. | MEDIUM |

**Why not the `islam` crate:** The `islam` crate bundles prayer times + Hijri dates together, but its prayer calculation is less proven than salah. Better to use best-in-class for each concern separately.

### Desktop Notifications
| Technology | Version | Purpose | Why | Confidence |
|------------|---------|---------|-----|------------|
| [notify-rust](https://github.com/hoodie/notify-rust) | 4.12.0 | Desktop notifications (libnotify/D-Bus) | The standard Rust notification crate. Targets XDG-compliant desktops (KDE, GNOME, XFCE, LXDE, Mate) via D-Bus. Supports urgency levels, timeouts, actions/callbacks, and persistent notifications. Pure Rust D-Bus client included. MIT/Apache-2.0. | HIGH |

### CLI & Configuration
| Technology | Version | Purpose | Why | Confidence |
|------------|---------|---------|-----|------------|
| [clap](https://docs.rs/clap/4.5.60) | 4.5.60 | CLI argument parsing | Industry standard. Derive macro for zero-boilerplate arg definitions. Supports subcommands, validation, shell completions. | HIGH |
| [serde](https://docs.rs/serde/1.0.228) | 1.0.228 | Serialization/deserialization framework | Required by toml for config struct mapping. Derive macros make config structs trivial. | HIGH |
| [toml](https://docs.rs/toml/1.0.6) | 1.0.6 | TOML config file parsing | Native Rust TOML encoder/decoder. Works with serde derives. Spec 1.1.0 compliant. | HIGH |
| [directories](https://docs.rs/directories/6.0.0) | 6.0.0 | XDG config directory resolution | Resolves `~/.config/` path correctly across platforms. Avoids hardcoding `$HOME`. Returns `ProjectDirs` for app-specific paths. | HIGH |

### Date/Time
| Technology | Version | Purpose | Why | Confidence |
|------------|---------|---------|-----|------------|
| [chrono](https://docs.rs/chrono/0.4.44) | 0.4.44 | Date, time, timezone handling | Both salah and hijri_date depend on chrono already. Use it consistently throughout. Provides `Local::now()`, timezone-aware datetimes, duration arithmetic. | HIGH |

## Full Dependency Summary

### Runtime Dependencies

```toml
[dependencies]
ratatui = "0.30"
crossterm = "0.29"
salah = "0.7"
hijri_date = "0.5"
notify-rust = "4.12"
clap = { version = "4.5", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
toml = "1.0"
directories = "6.0"
chrono = "0.4"
```

### Dev Dependencies

```toml
[dev-dependencies]
# None expected initially -- add as needed
```

## Alternatives Considered

| Category | Recommended | Alternative | Why Not |
|----------|-------------|-------------|---------|
| TUI framework | ratatui 0.30 | tui-rs | Deprecated. ratatui IS the maintained fork of tui-rs. |
| TUI framework | ratatui 0.30 | cursive | Widget-based model, less control over pixel-level layout needed for ASCII art clock |
| Terminal backend | crossterm | termion | crossterm is pure Rust, better Windows compat (if ever needed), more actively maintained |
| Prayer times | salah | praytime-rs | salah has stronger lineage (Adhan library, used in production mobile apps) |
| Prayer times | salah | Manual implementation | Unnecessary complexity when a correct library exists |
| Hijri calendar | hijri_date | islam crate | islam bundles too much; hijri_date is focused and does one thing well |
| Notifications | notify-rust | Raw `Command::new("notify-send")` | notify-rust handles D-Bus directly, works on desktops without notify-send binary, supports actions |
| Config | toml + serde | figment | Figment adds layered config (env vars, multiple files) -- overkill for a single TOML file with CLI overrides |
| Config | toml + serde | config crate | Similar to figment -- more machinery than needed |
| CLI parser | clap | argh | clap is the ecosystem standard, better docs, shell completions |
| Date/time | chrono | time | salah and hijri_date both depend on chrono; using time would mean two datetime libraries |

## What NOT to Use

| Technology | Why Not |
|------------|---------|
| `tui-rs` | Dead. Renamed/forked to ratatui. Do not depend on the original. |
| `termion` | Less maintained than crossterm, Linux-only (crossterm is more portable). |
| `islam` crate | Bundles prayer times + Hijri + Qibla. Jack of all trades, master of none. Use focused crates. |
| `reqwest` / any HTTP client | Project explicitly requires offline local calculation. No network dependencies. |
| `tokio` / async runtime | This is a synchronous TUI app with a simple event loop. Async adds complexity for no benefit. crossterm's `poll()`/`read()` is sufficient. |
| `figment` | Layered config framework is overkill. A single TOML file + clap overrides is trivially handled with serde. |

## Architecture Notes for Stack

**Event loop pattern:** Use crossterm's `event::poll(Duration)` + `event::read()` in a synchronous loop. Poll every 1 second to update the clock display. This is the standard ratatui pattern -- no async needed.

**Config layering (simple approach):**
1. Load defaults (hardcoded struct)
2. Read TOML file, merge over defaults with `Option<T>` fields
3. Apply CLI overrides from clap
No framework needed -- this is ~30 lines of Rust.

**ASCII art clock:** Build custom ratatui widget using `canvas::Canvas` or raw `Buffer` writes. Each digit is a 2D array of block characters. No external crate needed -- the digit patterns are small lookup tables.

**Terminal bell:** Use crossterm's `execute!(stdout(), Print("\x07"))` for BEL character. No crate needed.

## Risk Assessment

| Component | Risk | Mitigation |
|-----------|------|------------|
| salah (unmaintained since 2019) | May have edge-case bugs in extreme latitudes | Fork if needed; math is stable; test with known-good prayer time sources |
| hijri_date (small crate) | Limited year range (1356-1500 AH) | Covers 1938-2076 CE, sufficient for decades. If needed, Umm al-Qura tables are publicly available |
| ratatui API churn | Major versions may break widgets | Pin to 0.30.x; ratatui has good migration guides |
| notify-rust D-Bus | May fail in Wayland-only setups without D-Bus | XDG notification spec works on all major Linux DEs; fallback to terminal bell |

## Sources

- [ratatui docs (v0.30.0)](https://docs.rs/ratatui/0.30.0/ratatui/)
- [ratatui GitHub](https://github.com/ratatui/ratatui)
- [salah GitHub](https://github.com/insha/salah)
- [salah docs (v0.7.6)](https://docs.rs/salah/latest/salah/)
- [hijri_date docs (v0.5.1)](https://docs.rs/hijri_date/0.5.1/hijri_date/)
- [notify-rust GitHub](https://github.com/hoodie/notify-rust)
- [notify-rust docs (v4.12.0)](https://docs.rs/notify-rust/latest/notify_rust/)
- [clap docs (v4.5.60)](https://docs.rs/clap/4.5.60/clap/)
- [crossterm docs (v0.29.0)](https://docs.rs/crossterm/0.29.0/crossterm/)
- [praytime-rs docs (v1.0.1)](https://docs.rs/praytime-rs/latest/praytime_rs/)
- [PrayTimes.org calculation docs](https://praytimes.org/docs/calculation)
- [chrono docs (v0.4.44)](https://docs.rs/chrono/0.4.44/chrono/)
- [toml docs (v1.0.6)](https://docs.rs/toml/1.0.6/toml/)
- [serde docs (v1.0.228)](https://docs.rs/serde/1.0.228/serde/)
- [directories docs (v6.0.0)](https://docs.rs/directories/6.0.0/directories/)
