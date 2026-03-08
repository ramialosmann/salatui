# Architecture Patterns

**Domain:** Terminal-based Islamic prayer time clock (Rust/ratatui)
**Researched:** 2026-03-08

## Recommended Architecture

Use the **Elm Architecture (TEA)** pattern from the ratatui ecosystem. This is the recommended pattern for single-screen, state-driven TUI apps. The component architecture pattern is overkill for tui-adhan since it has one primary view with minimal interactive elements -- it is a display-focused clock, not a multi-pane interactive tool.

### High-Level Structure

```
main.rs
  |
  +-- app.rs          (Model: all application state)
  +-- event.rs        (Event polling: keyboard, tick timer)
  +-- ui.rs           (View: render state to terminal)
  +-- prayer.rs       (Prayer time calculation wrapper)
  +-- hijri.rs        (Hijri date conversion wrapper)
  +-- notification.rs (Desktop + terminal notifications)
  +-- config.rs       (TOML config loading + CLI args)
  +-- clock.rs        (Big digit rendering widget)
```

### Component Boundaries

| Component | Responsibility | Communicates With |
|-----------|---------------|-------------------|
| `main.rs` | Bootstrap: parse CLI, load config, init terminal, run event loop | app, event, ui, config |
| `app.rs` (Model) | Holds all state: current time, prayer times, countdown, notification flags | Updated by event loop, read by ui |
| `event.rs` | Polls keyboard input + emits tick events on interval (1 second) | Sends `Message` enum to update function |
| `ui.rs` (View) | Renders entire screen from `App` state using ratatui widgets | Reads app state, calls clock widget |
| `prayer.rs` | Wraps `salah` crate: calculates today's prayer schedule, determines current/next prayer | Called by app on date change or init |
| `hijri.rs` | Wraps `hijri_date` crate: converts today's Gregorian date to Hijri | Called by app on date change or init |
| `notification.rs` | Sends desktop notifications via `notify-rust`, triggers terminal bell | Called by update logic when prayer/pre-alert triggers |
| `config.rs` | Loads `~/.config/tui-adhan/config.toml`, merges CLI flag overrides | Called once at startup, produces `Config` struct |
| `clock.rs` | Custom big-digit widget or wrapper around `tui-big-text` | Called by ui.rs during render |

### The TEA Loop

```
                 +------------------+
                 |   Event Source   |
                 | (keyboard, tick) |
                 +--------+---------+
                          |
                     Message enum
                          |
                          v
              +-----------+-----------+
              |    update(app, msg)   |
              |  (pure state update)  |
              +-----------+-----------+
                          |
                   mutated App state
                          |
                          v
              +-----------+-----------+
              |    view(app, frame)   |
              |  (render to terminal) |
              +-----------+-----------+
```

## Data Flow

### Startup Flow

```
CLI args + config.toml
        |
        v
    Config struct
        |
        v
  Coordinates + Method + Madhab + Timezone
        |
        +-----> salah::PrayerSchedule::calculate() --> PrayerTimes for today
        |
        +-----> hijri_date::HijriDate::from_gr() --> Hijri date string
        |
        v
    App struct initialized with all computed data
        |
        v
    Enter event loop
```

### Per-Tick Flow (every 1 second)

```
Tick event arrives
      |
      v
  Update current_time (chrono::Local::now())
      |
      +---> Compute countdown = next_prayer_time - current_time
      |
      +---> Check: has the date rolled over midnight?
      |       YES --> recalculate prayer times + hijri date
      |
      +---> Check: has current_time crossed a prayer time?
      |       YES --> advance current/next prayer pointer
      |              trigger notification if not already sent
      |
      +---> Check: is current_time within pre-alert window?
      |       YES --> trigger pre-alert notification if not already sent
      |
      v
  Render updated state
```

### Notification Flow

```
update() detects prayer time crossing or pre-alert window
      |
      +---> Check notification_sent flags (prevent duplicates)
      |
      +---> Desktop: notify-rust Notification::new().summary().body().show()
      |
      +---> Terminal: print '\x07' (BEL) to trigger terminal bell/flash
      |
      +---> Set notification_sent flag for this prayer/alert
```

## Key Data Structures

### App (Model)

```rust
struct App {
    // Time state
    current_time: DateTime<Local>,

    // Prayer state
    prayer_times: PrayerTimes,       // from salah crate
    current_prayer: Prayer,          // enum: Fajr, Sunrise, Dhuhr, Asr, Maghrib, Isha
    next_prayer: Prayer,
    countdown: Duration,             // time until next prayer

    // Display state
    hijri_date: String,              // formatted Hijri date
    show_schedule: bool,             // toggle full schedule view

    // Notification tracking
    notifications_sent: HashSet<(Prayer, NaiveDate)>,  // prevent duplicate alerts
    pre_alerts_sent: HashSet<(Prayer, NaiveDate)>,

    // Config
    config: Config,

    // Control
    should_quit: bool,
}
```

### Message Enum

```rust
enum Message {
    Tick,                // 1-second timer
    Quit,                // q or Ctrl+C
    ToggleSchedule,      // show/hide full prayer schedule
    Resize(u16, u16),    // terminal resize
}
```

### Config

```rust
struct Config {
    latitude: f64,
    longitude: f64,
    calculation_method: Method,       // MWL, ISNA, Egyptian, UmmAlQura, etc.
    madhab: Madhab,                   // Shafi (standard) or Hanafi
    timezone: String,                 // e.g. "America/New_York"
    pre_alert_minutes: HashMap<Prayer, u32>,  // per-prayer pre-alert config
    notifications_enabled: bool,
    terminal_bell_enabled: bool,
}
```

## Screen Layout

```
+--------------------------------------------------+
|                                                    |
|           12 : 34 : 56                            |  <- Big ASCII digits (tui-big-text)
|                                                    |
|         15 Ramadan 1447 AH                        |  <- Hijri date
|                                                    |
|      Next: Dhuhr in 01:23:45                      |  <- Countdown
|                                                    |
+--------------------------------------------------+
|  Fajr    05:12  |  Dhuhr   12:58  |  Asr   16:22 |  <- Schedule (optional toggle)
|  Maghrib 19:05  |  Isha    20:30  |  Sunrise 6:42 |
+--------------------------------------------------+
```

The layout uses ratatui's `Layout::vertical()` to split into zones:
1. **Clock zone** (dominant, ~60% height): Big ASCII time via `tui-big-text`
2. **Info zone** (~20%): Hijri date + countdown text
3. **Schedule zone** (~20%, toggleable): 6 prayer times in a grid

## Patterns to Follow

### Pattern 1: Thin Library Wrappers

**What:** Wrap external crates (`salah`, `hijri_date`, `notify-rust`) behind thin modules with app-specific types.

**When:** Always, for all external dependencies.

**Why:** Isolates the app from crate API changes. If `salah` is unmaintained and needs replacement, only `prayer.rs` changes.

**Example:**
```rust
// prayer.rs - thin wrapper around salah crate
pub struct PrayerSchedule {
    pub fajr: DateTime<Local>,
    pub sunrise: DateTime<Local>,
    pub dhuhr: DateTime<Local>,
    pub asr: DateTime<Local>,
    pub maghrib: DateTime<Local>,
    pub isha: DateTime<Local>,
}

pub fn calculate(lat: f64, lon: f64, date: NaiveDate, method: Method, madhab: Madhab) -> PrayerSchedule {
    // salah crate calls here, map to our types
}

pub fn current_prayer(schedule: &PrayerSchedule, now: DateTime<Local>) -> Prayer { ... }
pub fn next_prayer(schedule: &PrayerSchedule, now: DateTime<Local>) -> (Prayer, DateTime<Local>) { ... }
```

### Pattern 2: Single-Second Tick as Heartbeat

**What:** The event loop sends a `Tick` message every 1 second. All time-dependent logic runs in the `Tick` handler.

**When:** For any TUI that displays continuously updating time.

**Why:** Keeps the render loop simple. No background threads for timers. The tick is the only source of time progression, making the app deterministic and easy to test.

### Pattern 3: Notification Deduplication via Sent-Flags

**What:** Track which notifications have been sent using a `HashSet<(Prayer, NaiveDate)>`. Check before sending, insert after.

**When:** Any notification-triggering logic that runs on a repeating timer.

**Why:** The tick fires every second, but a notification should fire once. Without dedup, you get 60+ duplicate notifications per minute.

## Anti-Patterns to Avoid

### Anti-Pattern 1: Calculating Prayer Times Every Tick

**What:** Calling `salah::PrayerSchedule::calculate()` on every 1-second tick.

**Why bad:** Prayer times change once per day. Recalculating sun position math 86,400 times a day wastes CPU and adds latency to every frame.

**Instead:** Calculate once at startup and once at midnight (date rollover). Cache the `PrayerTimes` in `App` state.

### Anti-Pattern 2: Spawning Threads for Notifications

**What:** Using `std::thread::spawn` or tokio tasks to schedule future notifications at exact prayer times.

**Why bad:** Adds concurrency complexity. Race conditions with the main event loop. The tick-based check is simpler and sufficient for 1-second precision.

**Instead:** Check in the tick handler: "has this prayer time passed? Has the notification been sent? If yes and no, send it now."

### Anti-Pattern 3: Storing Formatted Strings as State

**What:** Storing pre-rendered strings like `"01:23:45"` in the App model.

**Why bad:** Mixes model and view concerns. Makes testing harder. The view function should own formatting.

**Instead:** Store `Duration` or `DateTime` in the model. Format to string in `ui.rs` during render.

### Anti-Pattern 4: Using Full ICU4X for Hijri Dates

**What:** Pulling in the full `icu` crate just for Hijri date conversion.

**Why bad:** ICU4X is a massive dependency (Unicode CLDR data, multiple calendar systems, locale data). Binary size bloats from ~5MB to ~30MB+. Compile times increase significantly.

**Instead:** Use `hijri_date` crate -- it is tiny, focused, and does exactly Gregorian-to-Hijri conversion.

## Suggested Build Order

Dependencies flow top-to-bottom. Build in this order:

```
Phase 1: Foundation
  config.rs      (no deps, needed by everything)
  prayer.rs      (wraps salah, needs config for method/madhab)
  hijri.rs       (wraps hijri_date, standalone)

Phase 2: Core Loop
  app.rs         (Model struct, uses prayer + hijri + config)
  event.rs       (Tick + keyboard polling, standalone)
  main.rs        (event loop wiring, TEA pattern)

Phase 3: Display
  clock.rs       (big digit widget, uses tui-big-text)
  ui.rs          (full layout, reads App state, uses clock widget)

Phase 4: Notifications
  notification.rs (wraps notify-rust, triggered by app state)
```

**Rationale for this order:**
- Config, prayer calculation, and Hijri conversion are pure logic with no TUI dependency. They can be built and unit-tested independently.
- The core loop (App + events + main) is the skeleton. It can run headless or with a minimal render.
- Display is layered on once the data model is solid. Layout experimentation is faster when the data is already correct.
- Notifications are the last concern -- they are a side-effect bolted onto an already-working clock.

## Scalability Considerations

| Concern | At v1 (single user) | Future (if extended) |
|---------|---------------------|---------------------|
| Prayer times | Calculate once daily, cache | Same -- prayer math is date-bound |
| Timezone changes | Detect via `chrono::Local`, recalc if offset changes | Watch for TZ env var changes |
| Multiple locations | Not supported | Would need location selector in model, recalc on switch |
| Custom prayer adjustments | Config-level minute offsets per prayer | Same pattern, stored in Config |
| Binary size | ~5-8MB with ratatui + salah + hijri_date | Keep deps minimal, avoid ICU4X |

## Sources

- [Ratatui - The Elm Architecture](https://ratatui.rs/concepts/application-patterns/the-elm-architecture/) - HIGH confidence
- [Ratatui Component Architecture](https://ratatui.rs/concepts/application-patterns/component-architecture/) - HIGH confidence
- [Ratatui Templates](https://github.com/ratatui/templates) - HIGH confidence
- [salah crate (Islamic prayer times for Rust)](https://github.com/insha/salah) - HIGH confidence
- [salah docs.rs](https://docs.rs/salah/latest/salah/) - HIGH confidence
- [hijri_date crate](https://docs.rs/hijri_date/latest/hijri_date/) - HIGH confidence
- [tui-big-text widget](https://crates.io/crates/tui-big-text) - HIGH confidence
- [notify-rust crate](https://github.com/hoodie/notify-rust) - HIGH confidence
- [Ratatui Event Handling](https://ratatui.rs/concepts/event-handling/) - HIGH confidence
- [Prayer time calculation methods](https://praytimes.org/docs/calculation) - MEDIUM confidence
