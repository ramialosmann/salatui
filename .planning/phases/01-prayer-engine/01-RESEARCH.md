# Phase 1: Prayer Engine - Research

**Researched:** 2026-03-08
**Domain:** Islamic prayer time calculation, CLI argument parsing, TOML config management in Rust
**Confidence:** HIGH

## Summary

Phase 1 is a greenfield Rust project that needs to: (1) calculate Islamic prayer times from coordinates using the `salah` crate, (2) load/generate TOML config from `~/.config/tui-adhan/config.toml`, and (3) accept CLI flag overrides. The `salah` crate (v0.7.6, updated Jan 2026) provides a clean builder API for all 13 calculation methods, both madhabs, and built-in high-latitude rules -- it covers every CALC requirement out of the box. Config and CLI are standard Rust patterns using `clap` (derive), `serde`+`toml`, and `dirs` for XDG paths.

The main risk flagged in STATE.md -- that the `salah` crate was last updated in 2019 -- is now resolved: v0.7.6 was released January 2, 2026, and depends on chrono 0.4.42. The math is well-established astronomical calculation and the API is stable.

**Primary recommendation:** Use `salah` 0.7.6 for prayer calculation, `clap` 4.x derive for CLI, `toml`+`serde` for config, `dirs` 6.x for XDG paths. Structure as a binary crate with separate modules for config, prayer calculation, and CLI.

<user_constraints>

## User Constraints (from CONTEXT.md)

### Locked Decisions
- Default calculation method: MWL (Muslim World League)
- Default Asr madhab: Shafi (standard shadow ratio)
- Support all methods the salah crate provides out of the box (MWL, ISNA, Egyptian, Umm al-Qura, Karachi, etc.)
- No per-prayer minute adjustments -- deferred to v2 (DISP-08)
- Nested TOML sections: [location] for lat/lon, [calculation] for method/madhab
- When no config file exists, auto-generate default config at ~/.config/tui-adhan/config.toml with comments prompting user to set lat/lon
- Lat/lon are required -- app exits with clear error if missing after config generation
- CLI flags (--lat, --lon, --method, --madhab) fully override config values for that run
- Minimal plain text: prayer name and time, one per line, aligned columns
- Time format configurable: default 24h, config option to switch to 12h (e.g., time_format = "12h")
- No next-prayer highlighting -- just list all 6 times
- Today's date only -- no --date flag for other dates

### Claude's Discretion
- High-latitude fallback strategy (angle-based, 1/7 night, etc.)
- CLI argument parser choice (clap vs manual)
- Config deserialization approach
- Error message wording and exit codes

### Deferred Ideas (OUT OF SCOPE)
- Per-prayer minute adjustments to match local mosque times -- v2 (DISP-08)

</user_constraints>

<phase_requirements>

## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| CALC-01 | Calculate all 6 prayer times (Fajr, Sunrise, Dhuhr, Asr, Maghrib, Isha) from lat/lon and date | `salah` crate `PrayerSchedule` builder with `Prayer` enum covering all 6 + Qiyam |
| CALC-02 | User can select calculation method (MWL, ISNA, Egyptian, Umm al-Qura, Karachi, etc.) | `salah::Method` enum has 13 variants including all required methods |
| CALC-03 | User can select Asr madhab (Shafi or Hanafi) | `salah::Madhab` enum with `Shafi` and `Hanafi` variants |
| CALC-04 | App handles high-latitude locations safely with fallback methods | `salah::HighLatitudeRule` with 3 strategies + `recommended(coordinates)` auto-selector |
| CONF-01 | App reads settings from ~/.config/tui-adhan/config.toml | `dirs::config_dir()` + `toml`+`serde` deserialization |
| CONF-02 | User can override config values via CLI flags | `clap` derive with `Option<T>` fields merged over config values |
| CONF-03 | User can set Asr madhab in config | TOML `[calculation]` section with `madhab` field mapping to `Madhab` enum |

</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| salah | 0.7.6 | Prayer time calculation | Only actively maintained Rust prayer time library with full method/madhab/high-latitude support. Updated Jan 2026. |
| clap | 4.5.x | CLI argument parsing | De facto Rust CLI parser. Derive macro for zero-boilerplate argument structs. |
| serde | 1.0.x | Serialization framework | Universal Rust serialization. Required by `toml` crate. |
| toml | 1.0.x | TOML config parsing | Standard TOML serde integration. Handles read and write. |
| dirs | 6.0.0 | XDG directory resolution | Provides `config_dir()` returning `~/.config` on Linux. Tiny, well-maintained. |
| chrono | 0.4.x | Date/time handling | Required by `salah` (transitive dep). Use for local time conversion and formatting. |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| anyhow | 1.x | Error handling | Application-level error propagation with context |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| clap derive | manual arg parsing | clap adds compile time but gives --help, error messages, completions for free |
| dirs | hardcoded ~/.config | dirs handles XDG_CONFIG_HOME override correctly; hardcoding breaks XDG spec |
| anyhow | thiserror | thiserror better for libraries; anyhow better for binary crates |

**Installation:**
```toml
[dependencies]
salah = "0.7"
clap = { version = "4.5", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
toml = "1.0"
dirs = "6.0"
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1"
```

## Architecture Patterns

### Recommended Project Structure
```
src/
  main.rs          # Entry point: parse CLI, load config, calculate, print
  cli.rs           # Clap derive struct for CLI arguments
  config.rs        # Config struct, loading, generation, merging with CLI
  prayer.rs        # Wrapper around salah crate, prayer calculation logic
```

### Pattern 1: Config Loading with CLI Override
**What:** Load TOML config, then overlay CLI arguments where provided
**When to use:** Every run of the application
**Example:**
```rust
// cli.rs
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "tui-adhan", about = "Islamic prayer times")]
pub struct Cli {
    /// Latitude (overrides config)
    #[arg(long)]
    pub lat: Option<f64>,
    /// Longitude (overrides config)
    #[arg(long)]
    pub lon: Option<f64>,
    /// Calculation method (overrides config)
    #[arg(long)]
    pub method: Option<String>,
    /// Asr madhab: shafi or hanafi (overrides config)
    #[arg(long)]
    pub madhab: Option<String>,
}
```

```rust
// config.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    #[serde(default)]
    pub location: LocationConfig,
    #[serde(default)]
    pub calculation: CalculationConfig,
    #[serde(default)]
    pub display: DisplayConfig,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct LocationConfig {
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CalculationConfig {
    #[serde(default = "default_method")]
    pub method: String,
    #[serde(default = "default_madhab")]
    pub madhab: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DisplayConfig {
    #[serde(default = "default_time_format")]
    pub time_format: String,
}

fn default_method() -> String { "mwl".to_string() }
fn default_madhab() -> String { "shafi".to_string() }
fn default_time_format() -> String { "24h".to_string() }
```

### Pattern 2: Config Generation
**What:** Auto-generate a commented default config file when none exists
**When to use:** First run of the application
**Example:**
```rust
fn generate_default_config(path: &Path) -> anyhow::Result<()> {
    let default_content = r#"# tui-adhan configuration
# See: https://github.com/yourname/tui-adhan

[location]
# Required: Set your latitude and longitude
# lat = 40.7128
# lon = -74.0059

[calculation]
# Calculation method: mwl, egyptian, karachi, umm_al_qura, dubai,
#   moonsighting_committee, north_america, kuwait, qatar, singapore,
#   tehran, turkey
method = "mwl"

# Asr madhab: shafi or hanafi
madhab = "shafi"

[display]
# Time format: "24h" or "12h"
time_format = "24h"
"#;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, default_content)?;
    Ok(())
}
```

### Pattern 3: Prayer Calculation Wrapper
**What:** Thin wrapper over salah crate that accepts our config types
**When to use:** Core calculation logic
**Example:**
```rust
// prayer.rs
use salah::prelude::*;
use chrono::Local;

pub struct PrayerResult {
    pub fajr: chrono::DateTime<chrono::Local>,
    pub sunrise: chrono::DateTime<chrono::Local>,
    pub dhuhr: chrono::DateTime<chrono::Local>,
    pub asr: chrono::DateTime<chrono::Local>,
    pub maghrib: chrono::DateTime<chrono::Local>,
    pub isha: chrono::DateTime<chrono::Local>,
}

pub fn calculate_prayers(
    lat: f64,
    lon: f64,
    method: Method,
    madhab: Madhab,
) -> anyhow::Result<PrayerResult> {
    let coords = Coordinates::new(lat, lon);
    let date = Local::now().date_naive();
    let mut params = Configuration::with(method, madhab);
    params.high_latitude_rule(HighLatitudeRule::recommended(coords));
    let prayers = PrayerSchedule::new()
        .on(date)
        .for_location(coords)
        .with_configuration(params)
        .calculate()?;

    // Convert UTC times to local
    // prayers.time(Prayer::Fajr) returns DateTime<Utc>
    Ok(PrayerResult {
        fajr: prayers.time(Prayer::Fajr).with_timezone(&Local),
        sunrise: prayers.time(Prayer::Sunrise).with_timezone(&Local),
        dhuhr: prayers.time(Prayer::Dhuhr).with_timezone(&Local),
        asr: prayers.time(Prayer::Asr).with_timezone(&Local),
        maghrib: prayers.time(Prayer::Maghrib).with_timezone(&Local),
        isha: prayers.time(Prayer::Isha).with_timezone(&Local),
    })
}
```

### Pattern 4: String-to-Enum Mapping
**What:** Map user-facing config strings to salah enum variants
**When to use:** Config/CLI value interpretation
**Example:**
```rust
pub fn parse_method(s: &str) -> anyhow::Result<Method> {
    match s.to_lowercase().as_str() {
        "mwl" | "muslim_world_league" => Ok(Method::MuslimWorldLeague),
        "egyptian" => Ok(Method::Egyptian),
        "karachi" => Ok(Method::Karachi),
        "umm_al_qura" => Ok(Method::UmmAlQura),
        "dubai" => Ok(Method::Dubai),
        "moonsighting_committee" => Ok(Method::MoonsightingCommittee),
        "north_america" | "isna" => Ok(Method::NorthAmerica),
        "kuwait" => Ok(Method::Kuwait),
        "qatar" => Ok(Method::Qatar),
        "singapore" => Ok(Method::Singapore),
        "tehran" => Ok(Method::Tehran),
        "turkey" => Ok(Method::Turkey),
        _ => anyhow::bail!("Unknown calculation method: '{}'. Valid methods: mwl, egyptian, karachi, umm_al_qura, dubai, moonsighting_committee, north_america (isna), kuwait, qatar, singapore, tehran, turkey", s),
    }
}

pub fn parse_madhab(s: &str) -> anyhow::Result<Madhab> {
    match s.to_lowercase().as_str() {
        "shafi" | "shafii" | "standard" => Ok(Madhab::Shafi),
        "hanafi" => Ok(Madhab::Hanafi),
        _ => anyhow::bail!("Unknown madhab: '{}'. Valid options: shafi, hanafi", s),
    }
}
```

### Anti-Patterns to Avoid
- **Raw `unwrap()` on config loading:** Config file may not exist, be malformed, or have wrong permissions. Always use proper error handling with context.
- **Hardcoding `~/.config` path:** Use `dirs::config_dir()` to respect `XDG_CONFIG_HOME`.
- **Storing times as UTC without converting to local:** Users expect local time in output. Always convert before display.
- **Ignoring high-latitude rules:** Without a fallback strategy, prayer times at extreme latitudes produce NaN or panic. Always set a HighLatitudeRule.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Prayer time calculation | Solar position math | `salah` crate | Astronomical calculations are complex; angles, Julian dates, atmospheric refraction |
| CLI argument parsing | Manual arg iteration | `clap` derive | --help, error messages, type validation, completions for free |
| TOML parsing | Manual string parsing | `toml` + `serde` | Edge cases in TOML spec (multiline strings, inline tables, etc.) |
| XDG directory resolution | `format!("{}/.config", env::var("HOME")?)` | `dirs` crate | Handles XDG_CONFIG_HOME override, missing HOME, cross-platform |
| High-latitude prayer times | Custom angle interpolation | `salah::HighLatitudeRule` | Three established strategies already implemented and tested |

**Key insight:** The `salah` crate does the hard work (astronomical math). This phase is primarily plumbing: config in, prayer times out, formatted to stdout.

## Common Pitfalls

### Pitfall 1: salah `Configuration` is a builder, not a struct
**What goes wrong:** Trying to use `Configuration::with()` result directly as `Parameters`. The `with()` method returns `Configuration` (builder), and you may need `.done()` to get `Parameters`, or pass the builder directly to `with_configuration()`.
**Why it happens:** Builder pattern can be confusing; `Configuration` and `Parameters` are separate types.
**How to avoid:** Follow the exact builder chain: `Configuration::with(method, madhab)` then pass to `.with_configuration()`. If you need to set high latitude rule, call `.high_latitude_rule()` on the Configuration before passing it.
**Warning signs:** Type mismatch errors mentioning Parameters vs Configuration.

### Pitfall 2: UTC vs Local Time
**What goes wrong:** Printing prayer times in UTC instead of user's local timezone.
**Why it happens:** `salah` returns `DateTime<Utc>`. Developers forget to convert.
**How to avoid:** Always call `.with_timezone(&chrono::Local)` before formatting.
**Warning signs:** Prayer times off by several hours from expected values.

### Pitfall 3: Missing lat/lon Handling
**What goes wrong:** App panics or produces garbage times with 0.0/0.0 coordinates.
**Why it happens:** Default `Option<f64>` is None, but if someone sets lat=0, lon=0 (Gulf of Guinea), it is technically valid.
**How to avoid:** Treat `None` lat/lon as an error requiring user action. Validate that both lat AND lon are provided (not just one). Exit with a helpful message pointing to the config file.
**Warning signs:** Prayer times that don't match any real location.

### Pitfall 4: Config File Permissions
**What goes wrong:** App can't create config directory or write default config.
**Why it happens:** Permission issues, non-existent parent directories.
**How to avoid:** Use `create_dir_all` for the config directory. Handle write errors gracefully with a message suggesting manual creation.
**Warning signs:** Crashes on first run.

### Pitfall 5: salah `PrayerSchedule::calculate()` Returns Result
**What goes wrong:** Not handling the `Result` from `calculate()`.
**Why it happens:** Some coordinate/date combinations might fail.
**How to avoid:** Use `?` operator and provide context with `anyhow::Context`.
**Warning signs:** Unwrap panics with extreme coordinates.

## Code Examples

### Full Main Flow
```rust
// Source: Synthesized from salah docs + standard Rust patterns
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = load_or_create_config()?;
    let merged = merge_config_with_cli(config, &cli);

    let (lat, lon) = match (merged.location.lat, merged.location.lon) {
        (Some(lat), Some(lon)) => (lat, lon),
        _ => {
            eprintln!("Error: latitude and longitude are required.");
            eprintln!("Set them in your config file: {}", config_path()?.display());
            eprintln!("Or use: tui-adhan --lat 40.7128 --lon -74.0059");
            std::process::exit(1);
        }
    };

    let method = parse_method(&merged.calculation.method)?;
    let madhab = parse_madhab(&merged.calculation.madhab)?;
    let prayers = calculate_prayers(lat, lon, method, madhab)?;

    print_prayers(&prayers, &merged.display.time_format);
    Ok(())
}
```

### Output Formatting
```rust
fn print_prayers(prayers: &PrayerResult, time_format: &str) {
    let fmt = match time_format {
        "12h" => "%I:%M %p",
        _ => "%H:%M",
    };

    let times = [
        ("Fajr",    &prayers.fajr),
        ("Sunrise", &prayers.sunrise),
        ("Dhuhr",   &prayers.dhuhr),
        ("Asr",     &prayers.asr),
        ("Maghrib", &prayers.maghrib),
        ("Isha",    &prayers.isha),
    ];

    for (name, time) in &times {
        println!("{:<10} {}", name, time.format(fmt));
    }
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| salah 0.6.x | salah 0.7.6 | Jan 2026 | Updated chrono dependency, maintained API compatibility |
| clap 3.x derive | clap 4.5.x derive | 2023+ | Minor API changes; `#[clap(...)]` became `#[arg(...)]` and `#[command(...)]` |
| toml 0.5.x | toml 1.0.x | 2023+ | TOML spec 1.0 compliance, serde integration stable |

**Deprecated/outdated:**
- `clap` v2/v3 builder-style API: Still works but derive is the recommended approach for new projects
- `dirs` v4/v5: v6.0 is current, minor breaking changes in directory resolution

## Discretionary Recommendations

### High-Latitude Strategy
**Recommendation:** Use `HighLatitudeRule::recommended(coordinates)` which returns `MiddleOfTheNight` for latitudes below 48 degrees and `SeventhOfTheNight` for latitudes above. This is the safest default as it auto-adapts. No need to expose this as a user config option in Phase 1.

### CLI Parser
**Recommendation:** Use `clap` with derive macros. It provides `--help`, error messages, and type validation for free. The compile time cost is acceptable for a binary crate.

### Config Deserialization
**Recommendation:** Use `serde::Deserialize` + `serde::Serialize` derive on config structs with `#[serde(default)]` for optional sections. Use `toml::from_str` for reading and write the default config as a raw string literal (not serialized) to preserve comments.

### Exit Codes
**Recommendation:** Exit 0 on success, exit 1 on config errors (missing lat/lon), exit 2 on calculation errors. Use `std::process::exit()` only for the missing-lat/lon case; let `anyhow` handle other errors via `main() -> Result`.

## Open Questions

1. **salah `NorthAmerica` vs `ISNA` naming**
   - What we know: The `Method` enum uses `NorthAmerica` but ISNA (Islamic Society of North America) is the common name. The method uses 15-degree angles.
   - What's unclear: Whether this is exactly the ISNA method or a variation.
   - Recommendation: Accept both "isna" and "north_america" as aliases in config/CLI parsing.

2. **salah calculate() failure modes**
   - What we know: `calculate()` returns a Result type.
   - What's unclear: Exact conditions that cause failure (invalid coordinates? dates?). Docs have 42% coverage.
   - Recommendation: Wrap with anyhow context. Test with edge cases (poles, date line, far future dates) during implementation.

3. **Config struct reuse in Phase 2**
   - What we know: Phase 2 (TUI) and Phase 3 (notifications) will need the same config.
   - What's unclear: Whether additional config sections will be needed.
   - Recommendation: Design config struct to be extensible with `#[serde(default)]` on all sections. Future phases add sections without breaking existing configs.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in test framework (cargo test) |
| Config file | None -- Cargo.toml `[dev-dependencies]` section |
| Quick run command | `cargo test` |
| Full suite command | `cargo test -- --include-ignored` |

### Phase Requirements to Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| CALC-01 | Calculates all 6 prayer times from lat/lon/date | unit | `cargo test prayer::tests::test_calculates_all_six_prayers -- --exact` | No -- Wave 0 |
| CALC-02 | Different methods produce different Fajr/Isha times | unit | `cargo test prayer::tests::test_different_methods -- --exact` | No -- Wave 0 |
| CALC-03 | Shafi vs Hanafi produces different Asr time | unit | `cargo test prayer::tests::test_madhab_affects_asr -- --exact` | No -- Wave 0 |
| CALC-04 | High-latitude location does not NaN/panic | unit | `cargo test prayer::tests::test_high_latitude_no_panic -- --exact` | No -- Wave 0 |
| CONF-01 | Reads and parses config TOML correctly | unit | `cargo test config::tests::test_parse_config -- --exact` | No -- Wave 0 |
| CONF-02 | CLI flags override config values | unit | `cargo test config::tests::test_cli_overrides_config -- --exact` | No -- Wave 0 |
| CONF-03 | Madhab config value maps correctly | unit | `cargo test config::tests::test_madhab_config -- --exact` | No -- Wave 0 |

### Sampling Rate
- **Per task commit:** `cargo test`
- **Per wave merge:** `cargo test -- --include-ignored`
- **Phase gate:** Full suite green before `/gsd:verify-work`

### Wave 0 Gaps
- [ ] `src/prayer.rs` tests module -- covers CALC-01 through CALC-04
- [ ] `src/config.rs` tests module -- covers CONF-01 through CONF-03
- [ ] Cargo.toml project initialization with all dependencies

## Sources

### Primary (HIGH confidence)
- [salah docs.rs](https://docs.rs/salah/0.7.6/salah/) - Method enum (13 variants), Madhab enum, PrayerTimes struct, Configuration builder API
- [salah GitHub README](https://github.com/insha/salah) - Full code examples, HighLatitudeRule documentation, builder pattern usage
- [salah on lib.rs](https://lib.rs/crates/salah) - Version 0.7.6, released Jan 2, 2026, chrono 0.4.42 dependency

### Secondary (MEDIUM confidence)
- [clap on lib.rs](https://lib.rs/crates/clap) - Version 4.5.60, released Feb 19, 2026
- [toml on lib.rs](https://lib.rs/crates/toml) - Version 1.0.6, released Mar 6, 2026
- [dirs on lib.rs](https://lib.rs/crates/dirs) - Version 6.0.0, released Jan 12, 2025
- [serde on lib.rs](https://lib.rs/crates/serde) - Version 1.0.228

### Tertiary (LOW confidence)
- salah `calculate()` failure modes -- docs only 42% coverage, exact error conditions unverified

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - All libraries verified on lib.rs/docs.rs with current versions and active maintenance
- Architecture: HIGH - Standard Rust binary crate patterns, well-established config+CLI+library composition
- Pitfalls: MEDIUM - Based on API documentation review; some edge cases (calculate() failures) need hands-on validation
- salah API details: HIGH for Method/Madhab/HighLatitudeRule enums (verified via docs.rs); MEDIUM for PrayerTimes access patterns (docs 42% coverage)

**Research date:** 2026-03-08
**Valid until:** 2026-04-08 (stable domain, established libraries)
