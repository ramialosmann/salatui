# Phase 1: Prayer Engine - Context

**Gathered:** 2026-03-08
**Status:** Ready for planning

<domain>
## Phase Boundary

Config loading, CLI overrides, and accurate prayer time calculation for any location/method/madhab. User can run the app and see all 6 prayer times printed to stdout. No TUI, no notifications, no display formatting beyond plain text output.

</domain>

<decisions>
## Implementation Decisions

### Calculation methods & defaults
- Default calculation method: MWL (Muslim World League)
- Default Asr madhab: Shafi (standard shadow ratio)
- Support all methods the salah crate provides out of the box (MWL, ISNA, Egyptian, Umm al-Qura, Karachi, etc.)
- No per-prayer minute adjustments — deferred to v2 (DISP-08)

### Config file structure
- Nested TOML sections: [location] for lat/lon, [calculation] for method/madhab
- When no config file exists, auto-generate default config at ~/.config/tui-adhan/config.toml with comments prompting user to set lat/lon
- Lat/lon are required — app exits with clear error if missing after config generation
- CLI flags (--lat, --lon, --method, --madhab) fully override config values for that run

### CLI output format
- Minimal plain text: prayer name and time, one per line, aligned columns
- Time format configurable: default 24h, config option to switch to 12h (e.g., time_format = "12h")
- No next-prayer highlighting — just list all 6 times (highlighting comes in Phase 2)
- Today's date only — no --date flag for other dates

### Claude's Discretion
- High-latitude fallback strategy (angle-based, 1/7 night, etc.)
- CLI argument parser choice (clap vs manual)
- Config deserialization approach
- Error message wording and exit codes

</decisions>

<specifics>
## Specific Ideas

No specific requirements — open to standard approaches

</specifics>

<code_context>
## Existing Code Insights

### Reusable Assets
- No existing code — greenfield Rust project

### Established Patterns
- None yet — Phase 1 establishes the foundation patterns

### Integration Points
- Config struct will be shared with Phase 2 (TUI) and Phase 3 (notifications)
- Prayer calculation module will be called by Phase 2's event loop

</code_context>

<deferred>
## Deferred Ideas

- Per-prayer minute adjustments to match local mosque times — v2 (DISP-08)

</deferred>

---

*Phase: 01-prayer-engine*
*Context gathered: 2026-03-08*
