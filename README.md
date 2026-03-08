# tui-adhan

Islamic prayer times TUI clock for the terminal.

<!-- TODO: Add badges once published -->
<!-- [![Crates.io](https://img.shields.io/crates/v/tui-adhan)](https://crates.io/crates/tui-adhan) -->
<!-- [![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE) -->

<!-- screenshot: add terminal screenshot here before publishing -->
![tui-adhan screenshot](screenshot.png)

## Features

- Big ASCII art clock display (tty-clock style) with current time
- Live countdown to the next prayer with prayer name
- Hijri (Islamic calendar) date display alongside Gregorian date
- Full prayer schedule view showing all 6 prayer times (toggle with `s`)
- Desktop notifications via `notify-send` when a prayer time arrives
- Terminal bell/flash alerts at prayer time
- Configurable per-prayer pre-alert notifications (default 15 minutes before)
- 12 calculation methods covering major Islamic authorities worldwide
- Shafi and Hanafi Asr madhab support
- High-latitude safe calculation (MiddleOfTheNight rule)
- TOML configuration with auto-generation on first run
- CLI overrides for quick use without editing config

## Installation

### From crates.io

```sh
cargo install tui-adhan
```

### From source

```sh
git clone https://github.com/your-username/tui-adhan.git
cd tui-adhan
cargo build --release
```

The binary will be at `target/release/tui-adhan`.

### Requirements

- **Linux:** Requires `notify-send` (from `libnotify`) for desktop notifications.
  Install it via your package manager if not already present (e.g., `pacman -S libnotify` or `apt install libnotify-bin`).
  The app will still run without it, but desktop notifications will be disabled.

## Usage

Run with your config file (see Configuration below):

```sh
tui-adhan
```

Override location via CLI:

```sh
tui-adhan --lat 21.4225 --lon 39.8262
```

Override location, method, and madhab:

```sh
tui-adhan --lat 21.4225 --lon 39.8262 --method umm_al_qura --madhab hanafi
```

Latitude and longitude are required -- either set them in your config file or pass them via `--lat` and `--lon`.

### CLI Flags

| Flag       | Description                          |
|------------|--------------------------------------|
| `--lat`    | Latitude (overrides config)          |
| `--lon`    | Longitude (overrides config)         |
| `--method` | Calculation method (overrides config)|
| `--madhab` | Asr madhab: shafi or hanafi          |

## Configuration

On first run, a default config file is generated at:

```
~/.config/tui-adhan/config.toml
```

You must set your latitude and longitude before the app will work. Edit the config file and uncomment the `lat` and `lon` lines, or pass them via CLI flags.

### Default Config

```toml
# tui-adhan configuration

[location]
# Required: Set your latitude and longitude
# lat = 40.7128
# lon = -74.0059

[calculation]
# Calculation method. Options:
#   mwl              - Muslim World League
#   egyptian         - Egyptian General Authority of Survey
#   karachi          - University of Islamic Sciences, Karachi
#   umm_al_qura      - Umm al-Qura University, Makkah
#   dubai            - Dubai
#   moonsighting_committee - Moonsighting Committee
#   north_america    - Islamic Society of North America (ISNA)
#   kuwait           - Kuwait
#   qatar            - Qatar
#   singapore        - Singapore
#   tehran           - Institute of Geophysics, Tehran
#   turkey           - Diyanet Isleri Baskanligi, Turkey
method = "mwl"

# Asr madhab: shafi (standard) or hanafi
madhab = "shafi"

[display]
# Time format: "24h" or "12h"
time_format = "24h"

[notifications]
# Enable desktop notifications via notify-send
desktop = true

# Enable terminal bell (BEL character) at prayer time
bell = true

# Minutes before prayer time to send a pre-alert notification
pre_alert_minutes = 15

# Per-prayer pre-alert overrides (optional)
# Set to 0 to disable pre-alert for a specific prayer
# Sunrise is skipped by default (set a value to enable)
# [notifications.pre_alert]
# fajr = 20
# sunrise = 0
# dhuhr = 15
# asr = 15
# maghrib = 15
# isha = 15
```

## Calculation Methods

| Key                     | Method                                     |
|-------------------------|--------------------------------------------|
| `mwl`                   | Muslim World League                        |
| `egyptian`              | Egyptian General Authority of Survey       |
| `karachi`               | University of Islamic Sciences, Karachi    |
| `umm_al_qura`           | Umm al-Qura University, Makkah            |
| `dubai`                 | Dubai                                      |
| `moonsighting_committee`| Moonsighting Committee                     |
| `north_america` / `isna`| Islamic Society of North America           |
| `kuwait`                | Kuwait                                     |
| `qatar`                 | Qatar                                      |
| `singapore`             | Singapore                                  |
| `tehran`                | Institute of Geophysics, Tehran            |
| `turkey`                | Diyanet Isleri Baskanligi, Turkey          |

## Keybinds

| Key | Action                      |
|-----|-----------------------------|
| `s` | Toggle prayer schedule view |
| `q` | Quit                        |

## Dependencies

tui-adhan is built on:

- [salah](https://crates.io/crates/salah) -- prayer time calculation
- [ratatui](https://crates.io/crates/ratatui) -- terminal user interface
- [chrono](https://crates.io/crates/chrono) -- date and time handling
- [hijri_date](https://crates.io/crates/hijri_date) -- Hijri calendar conversion
- [clap](https://crates.io/crates/clap) -- CLI argument parsing

## License

MIT
