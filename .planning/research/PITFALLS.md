# Domain Pitfalls

**Domain:** Islamic prayer time TUI clock application
**Researched:** 2026-03-08

## Critical Pitfalls

Mistakes that cause incorrect prayer times, broken notifications, or require significant rework.

### Pitfall 1: Fixed Twilight Angle Assumption

**What goes wrong:** Using a single fixed solar depression angle (e.g., 18 degrees for Fajr) as if it produces accurate results everywhere. Observation reports confirm that the actual angle at which Fajr light appears varies by location, season, altitude, and atmospheric conditions. A 15-degree angle in one region may correspond to an 18-degree angle elsewhere.

**Why it happens:** Developers pick one calculation method (e.g., MWL at 18/17 degrees) and assume it works globally. The math is correct, but the Islamic definition of dawn ("when a line of light first appears and begins to spread across the horizon") does not map to a single universal angle.

**Consequences:** Users in certain locations report prayer times that are noticeably off from their local mosque schedule. This destroys trust in the app immediately -- prayer timing is not something users tolerate being "close enough."

**Prevention:**
- Support multiple calculation methods from day one (MWL, ISNA, Egyptian, Umm al-Qura, etc.) and make the method configurable.
- Default to a sensible method based on common regional usage, but never hard-code a single method.
- Allow manual minute-offset adjustments per prayer (e.g., +2 minutes to Fajr) so users can fine-tune to their local mosque.
- Document clearly which method is in use and what angles it employs.

**Detection:** Compare calculated times against aladhan.com API or local mosque schedules for the same location/date/method. Differences beyond 1-2 minutes indicate a bug.

**Phase:** Must be addressed in the core calculation phase. This is foundational.

---

### Pitfall 2: High Latitude Prayer Time Failure

**What goes wrong:** At latitudes above ~48.5 degrees N/S, twilight can persist throughout the night during summer months. The standard Fajr/Isha formulas produce NaN, negative values, or absurd times (e.g., Fajr at 1:00 AM, Isha at 11:30 PM) because the sun never dips below the required angle.

**Why it happens:** The twilight time formula `T(a) = 1/15 * arccos((-sin(a) - sin(L)*sin(D)) / (cos(L)*cos(D)))` becomes undefined when the argument to arccos falls outside [-1, 1]. Developers who test only at moderate latitudes (30-40 degrees) never encounter this.

**Consequences:** The app crashes, displays garbage times, or silently shows incorrect prayer schedules for users in northern Europe, Canada, Scandinavia, Russia, and parts of the UK -- all regions with significant Muslim populations.

**Prevention:**
- Implement at least one high-latitude adjustment method:
  - **Middle of the Night:** Split sunset-to-sunrise into halves; Isha before midpoint, Fajr after.
  - **One-Seventh of the Night:** Isha after first 1/7 of night, Fajr at last 1/7.
  - **Angle-Based:** Divide night proportionally by the ratio angle/60.
- The `salah` Rust crate already includes high-latitude handling, so using it avoids reimplementing this.
- Make the high-latitude method configurable.
- Add guard checks: if the arccos argument is out of range, fall back to the configured high-latitude method rather than panicking.

**Detection:** Test with coordinates for Reykjavik (64.1N), Oslo (59.9N), Stockholm (59.3N), London (51.5N) across June/July dates. If Fajr or Isha times are missing or absurd, the high-latitude handling is broken.

**Phase:** Must be handled in the calculation engine phase. Not a "later" feature -- it is a correctness requirement.

---

### Pitfall 3: Asr Juristic Method Mismatch

**What goes wrong:** The Asr prayer has two valid calculation methods based on Islamic jurisprudence: the majority position (Shafi'i, Maliki, Hanbali) uses shadow factor 1 (object's shadow equals its length plus noon shadow), while the Hanafi position uses shadow factor 2 (twice the length). This can produce a 30-60 minute difference in Asr time.

**Why it happens:** Developers pick one method without making it configurable, or worse, do not realize there are two valid methods.

**Consequences:** Hanafi-following users get incorrect Asr times (or vice versa). This is a dealbreaker for roughly 30% of the global Muslim population who follow the Hanafi school.

**Prevention:**
- Support both madhab options for Asr calculation from the start.
- Expose this as a clear config option: `madhab = "shafi"` or `madhab = "hanafi"`.
- The `salah` crate supports both via its `Madhab` enum.

**Detection:** Calculate Asr for the same location with both methods. The Hanafi time should always be later. If they are identical, one method is not implemented.

**Phase:** Core calculation phase. Must ship with both options.

---

### Pitfall 4: Desktop Notifications Failing Silently

**What goes wrong:** `notify-send` (or D-Bus notification calls) fail when the DBUS_SESSION_BUS_ADDRESS environment variable is not available, when no notification daemon is running, or when conflicting notification services are installed. The notification silently fails or hangs for 30 seconds waiting for a D-Bus timeout.

**Why it happens:** The TUI app runs in a terminal which is part of a graphical session, so this usually works. But edge cases abound:
- Running inside tmux/screen where the D-Bus session may not be forwarded.
- SSH sessions without D-Bus forwarding.
- Wayland compositors with different notification stacks.
- Multiple notification daemons conflicting (e.g., dunst vs. KDE notifications installed simultaneously).

**Consequences:** The user misses prayer notifications -- the primary value proposition of alerts. Since the failure is silent, they may not realize notifications are broken until they miss a prayer.

**Prevention:**
- Spawn `notify-send` as a non-blocking child process. Do not await its exit in the render loop.
- Catch and log notification failures visibly -- show a brief status message in the TUI itself ("Desktop notification failed").
- Always pair desktop notifications with an in-terminal notification (bell + visual flash) as a guaranteed fallback.
- Consider using the `notify-rust` crate for direct D-Bus communication instead of shelling out to `notify-send`, which gives better error handling.
- Test notification delivery on first launch and warn the user in the TUI if it fails.

**Detection:** Run the app inside tmux, inside a plain TTY (no X/Wayland), and on a system with no notification daemon. If it crashes or hangs, the notification code is not resilient enough.

**Phase:** Notification phase. Must be designed with fallback-first thinking.

---

### Pitfall 5: Hijri Date Off By One (or Two) Days

**What goes wrong:** The displayed Hijri date is one or two days different from what the user expects. Different communities, countries, and even apps show different Hijri dates for the same Gregorian date.

**Why it happens:** The Islamic calendar is fundamentally observation-based (new month starts when the crescent moon is sighted). Algorithmic conversion (tabular Hijri calendar or Umm al-Qura tables) produces an approximation. No algorithm can match every community's actual moon-sighting decisions. Additionally:
- Timezone handling errors cause the Hijri date to flip at the wrong time.
- Some algorithms define the Islamic day as starting at sunset (Maghrib), while others use midnight.
- The Umm al-Qura calendar (used in Saudi Arabia) can differ from tabular calculations by 1-2 days near month boundaries.

**Consequences:** Users see a Hijri date that disagrees with their community's calendar. During Ramadan, Eid, and Dhul Hijjah, this is highly visible and problematic.

**Prevention:**
- Use the Umm al-Qura calendar tables as the default -- it is the most widely recognized standard.
- Clearly label the Hijri date as "approximate" or "Umm al-Qura" in the UI or docs.
- Allow a manual +/- day adjustment in config for users whose community differs.
- Consider whether to flip the Hijri date at Maghrib (sunset) or midnight -- document the choice. Midnight is simpler and what most apps do; sunset is more technically correct Islamically.

**Detection:** Compare against islamicfinder.org or the Umm al-Qura official calendar for a sample of dates across different months. Check dates around known month transitions (1st Ramadan, 1st Shawwal).

**Phase:** Hijri display phase. Not as critical as prayer times, but must be addressed before release.

## Moderate Pitfalls

### Pitfall 6: DST Transition Bugs

**What goes wrong:** Prayer times jump by exactly one hour (forward or backward) on the day of a DST transition, or the countdown timer shows negative values, or the "next prayer" logic skips a prayer or shows yesterday's Isha.

**Why it happens:** The prayer calculation uses timezone offset, which changes during DST transitions. If the calculation caches the offset or calculates times at midnight using one offset but the actual prayer occurs after the DST change with a different offset, times are wrong. The Dhuhr formula is `12 + TimeZone - Lng/15 - EqT` -- if `TimeZone` is wrong by one hour, every prayer shifts by one hour.

**Prevention:**
- Use a proper timezone library (Rust `chrono-tz` or `jiff`) that handles DST transitions correctly. Never store raw UTC offsets -- store timezone names (e.g., "America/New_York").
- Recalculate prayer times fresh each day after midnight, using the correct offset for that date.
- For the countdown timer, always compare against UTC instants, not local wall-clock times.
- Test specifically on DST transition dates (second Sunday in March, first Sunday in November for US).

**Detection:** Set the system timezone to a DST-observant zone. Test on March and November transition dates. If any prayer time jumps by exactly 60 minutes compared to a reference, DST handling is broken.

**Phase:** Core calculation phase. Must use timezone-aware datetimes from the start.

---

### Pitfall 7: Midnight Rollover in Next-Prayer Logic

**What goes wrong:** After Isha, the "next prayer" should be Fajr of the next day. The countdown shows negative time, wraps around to 24 hours, or shows "0:00" because the next-prayer logic only looks at today's prayer times.

**Why it happens:** The developer calculates six prayer times for today and finds the next one after "now." After the last prayer (Isha), there is no "next" prayer today, so the logic fails.

**Consequences:** The primary value of the app ("how long until the next prayer") breaks every single night.

**Prevention:**
- Always calculate both today's and tomorrow's prayer times.
- The "next prayer" algorithm: iterate through today's times; if all have passed, use tomorrow's Fajr.
- The `salah` crate's `PrayerTimes` struct has `next_prayer()` and `current_prayer()` methods -- use them if they handle this correctly, but verify.

**Detection:** Run the app at 11 PM after Isha. If the countdown is wrong or missing, this bug is present.

**Phase:** Core display/countdown phase.

---

### Pitfall 8: Render Loop CPU Usage

**What goes wrong:** The TUI app consumes 5-15% CPU continuously because the render loop runs at 60 FPS (or higher) even though the display only changes once per second (the clock digits).

**Why it happens:** The default crossterm event loop polls rapidly. Developers use a tight loop with `poll(Duration::from_millis(16))` out of habit from interactive apps. A clock app does not need 60 FPS.

**Consequences:** Users running this in a persistent terminal pane (the stated use case) will see unnecessary battery drain on laptops and CPU waste.

**Prevention:**
- Use a 1-second tick interval for the main render loop. The clock only changes every second.
- Use crossterm's `event::poll(Duration::from_millis(1000))` or a tokio interval.
- Only re-render when: (a) a tick fires, (b) a key event occurs, or (c) a terminal resize event occurs.
- Profile with `top`/`htop` early. Target < 0.5% CPU at idle.

**Detection:** Run the app and check CPU usage with `top`. If it is above 1% while idle, the tick rate is too high.

**Phase:** TUI rendering phase. Design the event loop correctly from the start.

---

### Pitfall 9: Terminal Size Too Small for ASCII Clock

**What goes wrong:** The big ASCII digit display overflows or wraps when the terminal is smaller than expected, producing garbled output. Large ASCII digits (tty-clock style) typically need at least 60-70 columns and 10+ rows for the clock alone, plus space for the Hijri date and prayer schedule.

**Why it happens:** No minimum size check or graceful degradation. The app assumes a reasonable terminal size.

**Prevention:**
- Check terminal dimensions on startup and after resize events.
- Define minimum usable dimensions (e.g., 60x16).
- Implement graceful degradation: if the terminal is too small, show a compact view (simple digital time without ASCII art) rather than garbled output.
- Use ratatui's layout system to conditionally show/hide widgets based on available space.

**Detection:** Resize the terminal to 40x12 while the app is running. If it panics or displays garbage, this needs fixing.

**Phase:** TUI rendering phase.

## Minor Pitfalls

### Pitfall 10: Config File Error Handling

**What goes wrong:** A malformed TOML config file causes the app to crash with an unhelpful error message, or a missing config file causes a panic instead of using sensible defaults.

**Prevention:**
- Use `serde` with `#[serde(default)]` for all config fields so partial configs work.
- On first run with no config, generate a default config file with comments explaining each option.
- Print human-readable error messages for config parse failures, including the line number and what was expected.
- Validate lat/lon ranges, method names, and other values after parsing.

**Phase:** Configuration phase.

---

### Pitfall 11: Umm al-Qura Isha Adjustment During Ramadan

**What goes wrong:** When using the Umm al-Qura calculation method, Isha time during Ramadan should be fixed at 120 minutes after Maghrib (instead of the usual 90 minutes). Missing this produces incorrect Isha times for an entire month.

**Why it happens:** This is a special case specific to the Umm al-Qura method that is easy to overlook. The `salah` crate documentation explicitly notes: "you should add a +30 minute custom adjustment for Isha during Ramadan."

**Prevention:**
- If using the `salah` crate with Umm al-Qura method, detect Ramadan dates and apply the +30 minute Isha adjustment automatically.
- Test Isha times during Ramadan specifically when using Umm al-Qura.

**Phase:** Calculation engine phase, Umm al-Qura support.

---

### Pitfall 12: Sunrise Listed as a "Prayer"

**What goes wrong:** The app displays Sunrise in a way that implies it is a prayer time, confusing users. Or the app omits Sunrise, making users unable to know when the Fajr window ends (Fajr must be prayed before Sunrise).

**Prevention:**
- Display Sunrise but visually distinguish it from actual prayer times (different color, label it "Sunrise" not a prayer name).
- The countdown should skip Sunrise as a "next prayer" target -- after Fajr, the next prayer is Dhuhr, not Sunrise. However, optionally showing "Sunrise in X minutes" as supplementary info is helpful.

**Phase:** UI/display phase.

## Phase-Specific Warnings

| Phase Topic | Likely Pitfall | Mitigation |
|-------------|---------------|------------|
| Prayer calculation engine | High latitude NaN/panic | Guard arccos arguments, implement fallback methods |
| Prayer calculation engine | Wrong Asr for Hanafi users | Support both madhab options from day one |
| Prayer calculation engine | DST transition off-by-one-hour | Use timezone names not offsets, recalculate daily |
| Hijri date display | Off by one day vs. community | Use Umm al-Qura, allow manual +/- adjustment |
| TUI rendering | CPU waste from fast render loop | 1-second tick, event-driven re-render |
| TUI rendering | Garbled display on small terminals | Minimum size check, graceful degradation |
| Notifications | Silent failure of notify-send | Non-blocking spawn, in-terminal fallback, error reporting |
| Notifications | Umm al-Qura Isha in Ramadan | Auto-detect Ramadan, apply +30 min adjustment |
| Configuration | Crash on bad/missing config | Serde defaults, helpful error messages |
| Next-prayer logic | Broken countdown after Isha | Always calculate tomorrow's Fajr |

## Sources

- [PrayTimes.org Calculation Methods](https://praytimes.org/calculation) - Authoritative reference for prayer time formulas and high-latitude methods
- [Fiqh Council - Fifteen or Eighteen Degrees](https://fiqhcouncil.org/fifteen-or-eighteen-degrees-calculating-prayer-fasting-times-in-islam/) - Discussion of angle accuracy issues
- [AlAdhan Calculation Methods](https://aladhan.com/calculation-methods) - Overview of regional calculation methods
- [Astronomy Center - High Latitude Prayer Times](https://astronomycenter.net/latitude.html?l=en) - High latitude problem documentation
- [Salah Rust Crate (GitHub)](https://github.com/insha/salah) - Rust prayer time library, v0.7.6, based on Adhan/Astronomical Algorithms
- [Hijri-Gregorian Conversion Mismatch](https://prayertimesksa.com/hijri-gregorian-conversion-mismatch/) - Common conversion issues
- [Tabular Islamic Calendar (Wikipedia)](https://en.wikipedia.org/wiki/Tabular_Islamic_calendar) - Algorithmic calendar limitations
- [ArchWiki Desktop Notifications](https://wiki.archlinux.org/title/Desktop_notifications) - Linux notification system documentation
- [Ratatui FAQ](https://ratatui.rs/faq/) - Rendering and performance guidance
- [Ratatui Async Event Stream](https://ratatui.rs/tutorials/counter-async-app/async-event-stream/) - Event loop patterns
