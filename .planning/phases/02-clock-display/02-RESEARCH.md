# Phase 2: Clock Display - Research

**Researched:** 2026-03-09
**Domain:** Terminal UI (ratatui + crossterm), ASCII art clock, Hijri calendar
**Confidence:** HIGH

## Summary

This phase transforms tui-adhan from a print-and-exit CLI into a persistent terminal UI application. The core challenge is building a real-time updating TUI with large ASCII clock digits (tty-clock style), a prayer countdown timer, Hijri date display, and a toggleable schedule view -- all rendering at 1-second intervals.

The Rust TUI ecosystem is mature and well-documented. ratatui 0.30.0 is the current standard, includes crossterm by default, and provides the `ratatui::run()` convenience method that handles terminal setup/teardown. The hijri_date crate (0.5.1) provides Gregorian-to-Hijri conversion with English month names out of the box. The tty-clock digit style uses a simple 5-row-by-6-column boolean grid per digit, easily represented as const arrays in Rust.

**Primary recommendation:** Use ratatui 0.30.0 (bundles crossterm) with a poll-based event loop at ~250ms tick rate, rendering the clock from a custom ASCII digit widget, and hijri_date 0.5.1 for Islamic calendar conversion.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- tty-clock replica: same blocky ASCII digit style, same minimalist feel
- No border box around the clock -- bare digits, borderless mode
- HH:MM only (no seconds in the clock digits)
- Monochrome -- white/default terminal color only, no color
- Clock centered in the terminal
- Vertical stack, all centered: dates -> clock -> countdown
- Hijri + Gregorian date line at top: "14 Ramadan 1447 AH . 8 Mar 2026"
- Big ASCII clock digits in the middle
- Countdown below clock: "Maghrib in 1:23:45" (H:MM:SS format, ticks every second)
- 's' key toggles schedule view on/off
- 'q' key quits the app
- Schedule view replaces the clock (full screen takeover), not alongside it
- Hijri/Gregorian date line and countdown timer remain visible in schedule view
- Next prayer marked with arrow marker
- Past prayers shown in dim/gray text
- No keybinding help hints on screen
- Hijri format: "14 Ramadan 1447 AH" -- English month names, AH suffix
- Gregorian date shown alongside: "14 Ramadan 1447 AH . 8 Mar 2026" (single line, dot separator)
- Algorithmic Hijri conversion is fine -- no manual offset config needed

### Claude's Discretion
- ASCII digit font implementation (exact pixel pattern for each digit)
- Hijri conversion library choice
- ratatui widget structure and layout percentages
- Event loop implementation (crossterm vs other backends)
- Exact spacing between elements
- Error handling for terminal size too small

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| DISP-01 | App shows current time as big ASCII art digits (tty-clock style) | Custom widget using tty-clock 5x6 boolean grid font, rendered with block characters via ratatui Canvas or direct Buffer writes |
| DISP-02 | App shows countdown timer to next prayer with prayer name | Use chrono duration arithmetic on PrayerResult times; format as H:MM:SS; handle midnight rollover by recalculating for next day after Isha |
| DISP-03 | App shows Hijri (Islamic calendar) date | hijri_date 0.5.1 crate: `HijriDate::from_gr()` + `month_name_en()` for English month names |
| DISP-04 | User can toggle full schedule view showing all 6 prayer times for today | App state enum (Clock/Schedule), 's' key toggles, ratatui conditional rendering in draw function |
| DISP-05 | App updates display every second | crossterm poll-based event loop with 250ms tick rate, redraw every tick (4x/sec ensures smooth second transitions) |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| ratatui | 0.30.0 | TUI framework (rendering, layout, widgets) | De facto standard Rust TUI; bundles crossterm by default; `ratatui::run()` handles terminal init/cleanup |
| hijri_date | 0.5.1 | Gregorian-to-Hijri calendar conversion | Only maintained Rust crate for lunar Hijri; has `month_name_en()` for English names; covers years 1356-1500 AH (1938-2076 CE) |

### Already in Cargo.toml (reuse)
| Library | Version | Purpose | Used For |
|---------|---------|---------|----------|
| chrono | 0.4 | Date/time handling | `Local::now()`, duration arithmetic for countdown, date formatting |
| anyhow | 1 | Error handling | Terminal init errors, prayer calculation errors |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| hijri_date | islam crate (4.1.0) | islam crate is larger (full Islamic library with prayer times); hijri_date is focused and lightweight -- better fit since we already use salah for prayers |
| ratatui built-in crossterm | standalone crossterm 0.29.0 | ratatui 0.30.0 re-exports crossterm from its root -- no need for separate crossterm dependency |

**Installation:**
```bash
cargo add ratatui@0.30.0 hijri_date@0.5.1
```

**Note:** ratatui 0.30.0 enables the `crossterm` feature by default. No separate crossterm dependency needed. Access crossterm types via `ratatui::crossterm::*`.

## Architecture Patterns

### Recommended Project Structure
```
src/
├── main.rs          # Entry point: parse config, launch TUI or print-and-exit
├── cli.rs           # (existing) CLI argument parsing
├── config.rs        # (existing) Config loading
├── prayer.rs        # (existing) Prayer calculation + new: next_prayer() helper
├── app.rs           # NEW: App state struct, update logic, view toggling
├── tui.rs           # NEW: Terminal init/cleanup, event loop, tick handling
├── ui.rs            # NEW: All rendering (draw functions for clock view + schedule view)
└── digits.rs        # NEW: ASCII digit font data + rendering helper
```

### Pattern 1: App State Struct
**What:** Central struct holding all mutable application state.
**When to use:** Always -- ratatui's immediate-mode rendering reads from this every frame.
```rust
pub enum View {
    Clock,
    Schedule,
}

pub struct App {
    pub running: bool,
    pub view: View,
    pub prayers: PrayerResult,
    pub time_format: String,
    // Recalculation tracking
    pub prayer_date: chrono::NaiveDate,
}

impl App {
    pub fn next_prayer(&self) -> Option<(&str, chrono::DateTime<chrono::Local>)> {
        let now = chrono::Local::now();
        let prayers = [
            ("Fajr", self.prayers.fajr),
            ("Sunrise", self.prayers.sunrise),
            ("Dhuhr", self.prayers.dhuhr),
            ("Asr", self.prayers.asr),
            ("Maghrib", self.prayers.maghrib),
            ("Isha", self.prayers.isha),
        ];
        prayers.iter()
            .find(|(_, time)| *time > now)
            .map(|(name, time)| (*name, *time))
    }
}
```

### Pattern 2: Poll-Based Event Loop with Tick Rate
**What:** crossterm::event::poll() with a tick duration, ensuring regular redraws even without user input.
**When to use:** For any app that needs periodic updates (clocks, timers, animations).
```rust
use std::time::{Duration, Instant};

let tick_rate = Duration::from_millis(250); // 4 ticks/sec
let mut last_tick = Instant::now();

loop {
    // Draw current state
    terminal.draw(|f| ui::draw(f, &app))?;

    // Poll for events with remaining tick time
    let timeout = tick_rate.saturating_sub(last_tick.elapsed());
    if crossterm::event::poll(timeout)? {
        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            handle_key_event(&mut app, key);
        }
    }

    // Tick: update state
    if last_tick.elapsed() >= tick_rate {
        app.tick(); // check if prayers need recalculation, etc.
        last_tick = Instant::now();
    }

    if !app.running {
        break;
    }
}
```

### Pattern 3: Immediate-Mode Conditional Rendering
**What:** The draw function checks app state and renders the appropriate view each frame.
**When to use:** When you have multiple views (clock vs schedule).
```rust
fn draw(frame: &mut ratatui::Frame, app: &App) {
    // Layout: date line (top) | main content (middle) | countdown (bottom)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),    // date line
            Constraint::Fill(1),     // main content (clock or schedule)
            Constraint::Length(1),    // countdown line
        ])
        .split(frame.area());

    draw_date_line(frame, chunks[0], app);

    match app.view {
        View::Clock => draw_clock(frame, chunks[1], app),
        View::Schedule => draw_schedule(frame, chunks[1], app),
    }

    draw_countdown(frame, chunks[2], app);
}
```

### Pattern 4: tty-clock Digit Font as Const Arrays
**What:** Each digit (0-9) and colon represented as a 5-row x 6-column boolean grid, rendered using Unicode block characters.
**When to use:** For the big ASCII clock display.
```rust
// tty-clock uses a 5-high, 6-wide grid per digit
// Each row is a [bool; 6] where true = filled block
const DIGITS: [[[bool; 6]; 5]; 11] = [
    // 0
    [
        [false, true, true, true, true, false],
        [true, true, false, false, true, true],
        [true, true, false, false, true, true],
        [true, true, false, false, true, true],
        [false, true, true, true, true, false],
    ],
    // 1
    [
        [false, false, false, false, true, true],
        [false, false, false, false, true, true],
        [false, false, false, false, true, true],
        [false, false, false, false, true, true],
        [false, false, false, false, true, true],
    ],
    // ... digits 2-9 ...
    // 10 = colon
    [
        [false, false, false, false, false, false],
        [false, false, true, true, false, false],
        [false, false, false, false, false, false],
        [false, false, true, true, false, false],
        [false, false, false, false, false, false],
    ],
];

// Render: true cells become full-block character, false cells become space
const BLOCK: &str = "\u{2588}"; // Unicode full block character
```

### Anti-Patterns to Avoid
- **Separate crossterm dependency:** ratatui 0.30.0 re-exports crossterm. Adding a separate `crossterm` crate risks version mismatch.
- **Redrawing only on input:** A clock must redraw every second even with no input. Always use a tick-based loop, not pure event-driven.
- **Storing widgets between frames:** ratatui is immediate-mode. Reconstruct widgets from App state every frame.
- **Using `ratatui::run()` for this app:** The convenience `run()` method blocks on `event::read()` without tick support. For a clock app that updates every second, you need a custom event loop with `poll()`.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Hijri calendar conversion | Custom Tabular Islamic Calendar algorithm | `hijri_date` crate | Hijri calendar has complex month-length rules; edge cases around year boundaries; crate covers 1356-1500 AH |
| Terminal raw mode management | Manual enable/disable raw mode + alternate screen | `ratatui::init()` / `ratatui::restore()` | Handles cleanup on panic, signal handlers, alternate screen |
| Layout calculation | Manual coordinate math for centering | `ratatui::layout::Layout` with `Flex::Center` | Handles terminal resize, constraint solving |
| Unicode block rendering | Manual stdout writes with ANSI escapes | ratatui `Buffer::set_string()` or `Paragraph` widget | Proper double-width character handling, style support |

**Key insight:** ratatui handles all the hard parts of terminal rendering (raw mode, alternate screen, double-buffering, resize). Focus implementation effort on the app logic and digit font, not terminal plumbing.

## Common Pitfalls

### Pitfall 1: Midnight Prayer Rollover
**What goes wrong:** After Isha, there is no "next prayer" today. The countdown shows nothing or panics.
**Why it happens:** `next_prayer()` only searches today's prayer times.
**How to avoid:** When no prayer is found after Isha, calculate tomorrow's Fajr. Store the next-day calculation or recalculate prayers when the date changes (at midnight).
**Warning signs:** Countdown disappears or shows negative time after Isha.

### Pitfall 2: Terminal Too Small for Clock Digits
**What goes wrong:** ASCII art clock digits overflow the terminal width, causing rendering artifacts.
**Why it happens:** HH:MM with colon = 5 digits, each 6 chars wide + spacing = ~34+ columns minimum. Small terminals break layout.
**How to avoid:** Check `frame.area().width` and `frame.area().height` before rendering clock. Show a "terminal too small" message if below minimum dimensions (roughly 40x10).
**Warning signs:** Garbled display, panic on out-of-bounds buffer write.

### Pitfall 3: Forgetting Terminal Cleanup on Panic
**What goes wrong:** If the app panics, the terminal stays in raw mode with alternate screen. User's shell is broken.
**Why it happens:** Raw mode / alternate screen not restored on unexpected exit.
**How to avoid:** Use `ratatui::init()` and `ratatui::restore()` which install panic hooks. Or set a custom panic hook that calls `ratatui::restore()`.
**Warning signs:** Terminal broken after Ctrl+C or panic.

### Pitfall 4: Hijri Date Off by One
**What goes wrong:** The Hijri date shown is off by 1-2 days from actual Islamic calendar.
**Why it happens:** hijri_date uses the Kuwaiti algorithmic calendar, which can differ from official sighting-based calendars by 1-2 days.
**How to avoid:** This is acceptable per user decision ("algorithmic conversion is fine"). Document that it is approximate. The crate is consistent and deterministic.
**Warning signs:** User complaints about Hijri date not matching local mosque calendar (expected, not a bug).

### Pitfall 5: Flickering Display
**What goes wrong:** The screen flickers on each redraw.
**Why it happens:** Naive rendering that clears and redraws entire screen. Or rendering too infrequently so updates look jerky.
**How to avoid:** ratatui uses double-buffering by default (only diffs are sent to terminal). Keep tick rate at 250ms (4/sec). Do not call `terminal.clear()` inside the loop.
**Warning signs:** Visible flicker, especially on slower terminals.

### Pitfall 6: Edition 2024 Compatibility
**What goes wrong:** New dependencies may not compile with Rust edition 2024.
**Why it happens:** The project uses `edition = "2024"` which is very new. Some crates may not have updated.
**How to avoid:** Test compilation early. ratatui 0.30.0 and hijri_date 0.5.1 should work fine since edition is per-crate, not transitive. But verify with `cargo check` after adding dependencies.
**Warning signs:** Compilation errors mentioning edition features.

## Code Examples

### Hijri Date Formatting
```rust
// Source: docs.rs/hijri_date/0.5.1
use hijri_date::HijriDate;
use chrono::Local;

fn format_date_line() -> String {
    let now = Local::now();
    let hd = HijriDate::from_gr(
        now.year() as u16,   // Note: from_gr takes u16 for year
        now.month() as u8,
        now.day() as u8,
    ).expect("date in valid range");

    let hijri = format!("{} {} {} AH",
        hd.day(),
        hd.month_name_en(),
        hd.year(),
    );
    let greg = now.format("%-d %b %Y").to_string();
    format!("{} \u{00B7} {}", hijri, greg)  // middle dot separator
}
```

### Next Prayer with Countdown
```rust
use chrono::{DateTime, Local};

fn format_countdown(prayers: &PrayerResult) -> String {
    let now = Local::now();
    let all = [
        ("Fajr", prayers.fajr),
        ("Sunrise", prayers.sunrise),
        ("Dhuhr", prayers.dhuhr),
        ("Asr", prayers.asr),
        ("Maghrib", prayers.maghrib),
        ("Isha", prayers.isha),
    ];

    if let Some((name, time)) = all.iter().find(|(_, t)| *t > now) {
        let remaining = *time - now;
        let total_secs = remaining.num_seconds();
        let h = total_secs / 3600;
        let m = (total_secs % 3600) / 60;
        let s = total_secs % 60;
        format!("{} in {}:{:02}:{:02}", name, h, m, s)
    } else {
        // After Isha: need tomorrow's Fajr
        "Fajr in --:--:--".to_string() // placeholder; real impl recalculates
    }
}
```

### Rendering Clock Digits to ratatui Buffer
```rust
use ratatui::prelude::*;
use ratatui::buffer::Buffer;

fn render_digit(buf: &mut Buffer, digit: usize, x: u16, y: u16, style: Style) {
    let pattern = &DIGITS[digit];
    for (row, cols) in pattern.iter().enumerate() {
        for (col, &filled) in cols.iter().enumerate() {
            let ch = if filled { "\u{2588}" } else { " " };
            let pos_x = x + col as u16;
            let pos_y = y + row as u16;
            if let Some(cell) = buf.cell_mut((pos_x, pos_y)) {
                cell.set_symbol(ch);
                cell.set_style(style);
            }
        }
    }
}
```

### Key Event Handling
```rust
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};

fn handle_key_event(app: &mut App, key: event::KeyEvent) {
    // Only handle key press events (not release/repeat)
    if key.kind != KeyEventKind::Press {
        return;
    }
    match key.code {
        KeyCode::Char('q') => app.running = false,
        KeyCode::Char('s') => {
            app.view = match app.view {
                View::Clock => View::Schedule,
                View::Schedule => View::Clock,
            };
        }
        _ => {}
    }
}
```

### Schedule View Rendering
```rust
fn draw_schedule(frame: &mut Frame, area: Rect, app: &App) {
    let now = Local::now();
    let prayers = [
        ("Fajr", app.prayers.fajr),
        ("Sunrise", app.prayers.sunrise),
        ("Dhuhr", app.prayers.dhuhr),
        ("Asr", app.prayers.asr),
        ("Maghrib", app.prayers.maghrib),
        ("Isha", app.prayers.isha),
    ];

    let mut lines = vec![
        Line::from("Today's Prayer Times").centered(),
        Line::from("---"),
    ];

    let next = prayers.iter().find(|(_, t)| *t > now);
    let fmt = match app.time_format.as_str() {
        "12h" => "%I:%M %p",
        _ => "%H:%M",
    };

    for (name, time) in &prayers {
        let time_str = time.format(fmt).to_string();
        let is_past = *time <= now;
        let is_next = next.map_or(false, |(n, _)| *n == *name);

        let marker = if is_next { "\u{25B6} " } else { "  " };
        let style = if is_past {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default()
        };

        lines.push(Line::from(
            Span::styled(format!("{}{:<10} {}", marker, name, time_str), style)
        ));
    }

    let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
    // Center vertically by calculating offset
    let content_height = 2 + 6; // title + separator + 6 prayers
    let vertical = Layout::vertical([Constraint::Fill(1), Constraint::Length(content_height), Constraint::Fill(1)])
        .split(area);
    frame.render_widget(paragraph, vertical[1]);
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `tui-rs` crate | `ratatui` (fork) | 2023 | tui-rs is unmaintained; ratatui is the active successor |
| Manual terminal init/cleanup | `ratatui::init()` / `ratatui::restore()` | ratatui 0.28+ | Handles panic hooks, alternate screen, raw mode automatically |
| Separate crossterm dependency | ratatui re-exports crossterm | ratatui 0.27+ | No version mismatch risk; use `ratatui::crossterm::*` |
| `ratatui::run()` for simple apps | Custom event loop for ticking apps | ratatui 0.30 | `run()` is great for simple apps but blocks on read(); clock apps need poll() |

**Deprecated/outdated:**
- `tui-rs`: Unmaintained, use ratatui instead
- Manual `CrosstermBackend::new()` + `enable_raw_mode()`: Use `ratatui::init()` instead

## Open Questions

1. **hijri_date parameter types**
   - What we know: `from_gr()` takes year/month/day but docs show u16/u8/u8. chrono gives i32/u32/u32.
   - What's unclear: Exact type conversions needed, potential truncation.
   - Recommendation: Cast with `as u16` / `as u8` after validating ranges. The crate covers 1938-2076 CE so current dates are safe.

2. **Exact tty-clock digit patterns**
   - What we know: tty-clock uses a 5-row grid with 2-wide block characters per cell (so 6 columns effectively = 3 block pairs).
   - What's unclear: The exact boolean values for each digit 0-9. Cannot fetch source due to rate limiting.
   - Recommendation: Reference the tty-clock C source (`ttyclock.h` number array) during implementation or recreate from screenshots. The pattern is simple: 7-segment-display style using filled blocks. This is Claude's discretion per CONTEXT.md.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in test framework (cargo test) |
| Config file | none (built-in, no config needed) |
| Quick run command | `cargo test` |
| Full suite command | `cargo test` |

### Phase Requirements to Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| DISP-01 | ASCII digits render correctly for all digits 0-9 and colon | unit | `cargo test digits` | No -- Wave 0 |
| DISP-02 | Next prayer calculation returns correct prayer and countdown formats correctly | unit | `cargo test next_prayer` | No -- Wave 0 |
| DISP-03 | Hijri date formatting produces expected string format | unit | `cargo test hijri` | No -- Wave 0 |
| DISP-04 | View toggle switches between Clock and Schedule | unit | `cargo test view_toggle` | No -- Wave 0 |
| DISP-05 | App renders without panic (smoke test with mock terminal) | unit | `cargo test render` | No -- Wave 0 |

### Sampling Rate
- **Per task commit:** `cargo test`
- **Per wave merge:** `cargo test`
- **Phase gate:** Full suite green before `/gsd:verify-work`

### Wave 0 Gaps
- [ ] `src/digits.rs` -- unit tests for digit pattern correctness and rendering bounds
- [ ] `src/app.rs` -- unit tests for next_prayer(), view toggle, midnight rollover
- [ ] Test for Hijri date formatting (can be in app.rs or a dedicated test)
- [ ] Note: TUI rendering is hard to unit test beyond "does not panic". Visual correctness is manual.

## Sources

### Primary (HIGH confidence)
- [ratatui 0.30.0](https://ratatui.rs/) - Installation, layout, rendering concepts, event handling
- [hijri_date 0.5.1](https://docs.rs/hijri_date/0.5.1/) - API docs, HijriDate struct, month_name_en(), from_gr()
- [crossterm 0.29.0](https://docs.rs/crossterm/) - Event handling, poll/read, KeyEvent (bundled via ratatui)
- cargo search -- verified current crate versions

### Secondary (MEDIUM confidence)
- [ratatui event handling](https://ratatui.rs/concepts/event-handling/) - Event loop patterns
- [ratatui layout](https://ratatui.rs/concepts/layout/) - Flex::Center, constraints, nested layouts
- [ratatui rendering](https://ratatui.rs/concepts/rendering/) - Immediate mode, Frame, double-buffering

### Tertiary (LOW confidence)
- tty-clock digit patterns -- could not fetch source code directly; digit grid described from training data knowledge of the tty-clock `ttyclock.h` number array. Verify against actual source during implementation.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - ratatui 0.30.0 and hijri_date 0.5.1 verified via cargo search and official docs
- Architecture: HIGH - ratatui patterns well-documented; poll-based event loop is standard for clock apps
- Pitfalls: HIGH - midnight rollover, terminal cleanup, and flickering are well-known TUI concerns
- Digit font: MEDIUM - tty-clock pattern described from training data; exact values need source verification

**Research date:** 2026-03-09
**Valid until:** 2026-04-09 (stable ecosystem, 30-day validity)
