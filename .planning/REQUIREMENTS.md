# Requirements: tui-adhan

**Defined:** 2026-03-08
**Core Value:** Always show the user exactly how long until the next prayer — at a glance, from across the room.

## v1 Requirements

Requirements for initial release. Each maps to roadmap phases.

### Prayer Calculation

- [ ] **CALC-01**: App calculates all 6 prayer times (Fajr, Sunrise, Dhuhr, Asr, Maghrib, Isha) from lat/lon and date
- [ ] **CALC-02**: User can select calculation method (MWL, ISNA, Egyptian, Umm al-Qura, Karachi, etc.)
- [ ] **CALC-03**: User can select Asr madhab (Shafi or Hanafi)
- [ ] **CALC-04**: App handles high-latitude locations safely with fallback methods (no NaN/panic)

### Display

- [ ] **DISP-01**: App shows current time as big ASCII art digits (tty-clock style)
- [ ] **DISP-02**: App shows countdown timer to next prayer with prayer name
- [ ] **DISP-03**: App shows Hijri (Islamic calendar) date
- [ ] **DISP-04**: User can toggle full schedule view showing all 6 prayer times for today
- [ ] **DISP-05**: App updates display every second

### Configuration

- [ ] **CONF-01**: App reads settings from ~/.config/tui-adhan/config.toml
- [ ] **CONF-02**: User can override config values via CLI flags (--lat, --lon, --method, etc.)
- [ ] **CONF-03**: User can set Asr madhab in config
- [ ] **CONF-04**: User can configure per-prayer pre-alert minutes in config

### Notifications

- [ ] **NOTF-01**: App sends desktop notification (notify-send) when a prayer time arrives
- [ ] **NOTF-02**: App triggers terminal bell/flash when a prayer time arrives
- [ ] **NOTF-03**: App sends pre-alert notification X minutes before each prayer (configurable per-prayer)

## v2 Requirements

Deferred to future release. Tracked but not in current roadmap.

### Display Enhancements

- **DISP-06**: Responsive layout adapts to different terminal sizes
- **DISP-07**: Configurable color themes
- **DISP-08**: Per-prayer minute adjustments to match local mosque times

### Notifications Enhancements

- **NOTF-04**: Sound/adhan audio playback at prayer time

## Out of Scope

| Feature | Reason |
|---------|--------|
| Qibla direction | Keeping v1 focused on time display |
| Sound/adhan playback | Adds audio dependency complexity, defer to v2 |
| API-based prayer times | Local calculation preferred for offline reliability |
| Mobile or GUI version | Terminal-only project |
| Prayer tracking/logging | Out of scope for a clock app |
| Quran content | Scope creep — different app |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| CALC-01 | — | Pending |
| CALC-02 | — | Pending |
| CALC-03 | — | Pending |
| CALC-04 | — | Pending |
| DISP-01 | — | Pending |
| DISP-02 | — | Pending |
| DISP-03 | — | Pending |
| DISP-04 | — | Pending |
| DISP-05 | — | Pending |
| CONF-01 | — | Pending |
| CONF-02 | — | Pending |
| CONF-03 | — | Pending |
| CONF-04 | — | Pending |
| NOTF-01 | — | Pending |
| NOTF-02 | — | Pending |
| NOTF-03 | — | Pending |

**Coverage:**
- v1 requirements: 16 total
- Mapped to phases: 0
- Unmapped: 16

---
*Requirements defined: 2026-03-08*
*Last updated: 2026-03-08 after initial definition*
