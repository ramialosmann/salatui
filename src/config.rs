use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::cli::Cli;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    #[serde(default)]
    pub location: LocationConfig,
    #[serde(default)]
    pub calculation: CalculationConfig,
    #[serde(default)]
    pub display: DisplayConfig,
    #[serde(default)]
    pub notifications: NotificationConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        toml::from_str("").expect("empty string should parse to defaults")
    }
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

impl Default for CalculationConfig {
    fn default() -> Self {
        Self {
            method: default_method(),
            madhab: default_madhab(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DisplayConfig {
    #[serde(default = "default_time_format")]
    pub time_format: String,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            time_format: default_time_format(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NotificationConfig {
    #[serde(default = "default_true")]
    pub desktop: bool,
    #[serde(default = "default_true")]
    pub bell: bool,
    #[serde(default = "default_pre_alert_minutes")]
    pub pre_alert_minutes: u32,
    #[serde(default)]
    pub pre_alert: PreAlertConfig,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            desktop: default_true(),
            bell: default_true(),
            pre_alert_minutes: default_pre_alert_minutes(),
            pre_alert: PreAlertConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct PreAlertConfig {
    pub fajr: Option<u32>,
    pub sunrise: Option<u32>,
    pub dhuhr: Option<u32>,
    pub asr: Option<u32>,
    pub maghrib: Option<u32>,
    pub isha: Option<u32>,
}

impl PreAlertConfig {
    /// Returns the pre-alert minutes for a given prayer name.
    /// Sunrise defaults to 0 (skip) unless explicitly set.
    /// All other prayers fall back to the global default.
    pub fn get_minutes(&self, prayer_name: &str, global_default: u32) -> u32 {
        match prayer_name {
            "Fajr" => self.fajr.unwrap_or(global_default),
            "Sunrise" => self.sunrise.unwrap_or(0),
            "Dhuhr" => self.dhuhr.unwrap_or(global_default),
            "Asr" => self.asr.unwrap_or(global_default),
            "Maghrib" => self.maghrib.unwrap_or(global_default),
            "Isha" => self.isha.unwrap_or(global_default),
            _ => global_default,
        }
    }
}

fn default_true() -> bool {
    true
}

fn default_pre_alert_minutes() -> u32 {
    15
}

fn default_method() -> String {
    "mwl".to_string()
}

fn default_madhab() -> String {
    "shafi".to_string()
}

fn default_time_format() -> String {
    "24h".to_string()
}

/// Returns the path to the config file: ~/.config/tui-adhan/config.toml
pub fn config_path() -> anyhow::Result<PathBuf> {
    let config_dir =
        dirs::config_dir().ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
    Ok(config_dir.join("tui-adhan").join("config.toml"))
}

/// Generates a default config file with helpful comments at the given path.
pub fn generate_default_config(path: &Path) -> anyhow::Result<()> {
    let default_content = r#"# tui-adhan configuration

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
"#;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, default_content)?;
    Ok(())
}

/// Loads the config file if it exists, or generates a default one first.
pub fn load_or_create_config() -> anyhow::Result<AppConfig> {
    let path = config_path()?;

    if !path.exists() {
        eprintln!(
            "No config file found. Generating default at: {}",
            path.display()
        );
        generate_default_config(&path)?;
    }

    let content = std::fs::read_to_string(&path)
        .map_err(|e| anyhow::anyhow!("Failed to read config file at {}: {}", path.display(), e))?;

    let config: AppConfig = toml::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Failed to parse config file at {}: {}", path.display(), e))?;

    Ok(config)
}

/// Merges CLI arguments over config values. CLI `Some` values take precedence.
pub fn merge_config_with_cli(mut config: AppConfig, cli: &Cli) -> AppConfig {
    if cli.lat.is_some() {
        config.location.lat = cli.lat;
    }
    if cli.lon.is_some() {
        config.location.lon = cli.lon;
    }
    if let Some(ref method) = cli.method {
        config.calculation.method = method.clone();
    }
    if let Some(ref madhab) = cli.madhab {
        config.calculation.madhab = madhab.clone();
    }
    config
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        let toml_str = r#"
[location]
lat = 21.4225
lon = 39.8262

[calculation]
method = "umm_al_qura"
madhab = "hanafi"

[display]
time_format = "12h"
"#;
        let config: AppConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.location.lat, Some(21.4225));
        assert_eq!(config.location.lon, Some(39.8262));
        assert_eq!(config.calculation.method, "umm_al_qura");
        assert_eq!(config.calculation.madhab, "hanafi");
        assert_eq!(config.display.time_format, "12h");
    }

    #[test]
    fn test_parse_config_defaults() {
        let config: AppConfig = toml::from_str("").unwrap();
        assert_eq!(config.location.lat, None);
        assert_eq!(config.location.lon, None);
        assert_eq!(config.calculation.method, "mwl");
        assert_eq!(config.calculation.madhab, "shafi");
        assert_eq!(config.display.time_format, "24h");
    }

    #[test]
    fn test_cli_overrides_config() {
        let config: AppConfig = toml::from_str(
            r#"
[calculation]
method = "mwl"
madhab = "shafi"
"#,
        )
        .unwrap();

        let cli = Cli {
            lat: Some(40.7128),
            lon: None,
            method: Some("egyptian".to_string()),
            madhab: None,
        };

        let merged = merge_config_with_cli(config, &cli);
        assert_eq!(merged.location.lat, Some(40.7128));
        assert_eq!(merged.location.lon, None);
        assert_eq!(merged.calculation.method, "egyptian");
        assert_eq!(merged.calculation.madhab, "shafi");
    }

    #[test]
    fn test_madhab_config() {
        let toml_str = r#"
[calculation]
madhab = "hanafi"
"#;
        let config: AppConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.calculation.madhab, "hanafi");
    }

    #[test]
    fn test_notification_defaults() {
        let config: NotificationConfig = toml::from_str("").unwrap();
        assert_eq!(config.desktop, true);
        assert_eq!(config.bell, true);
        assert_eq!(config.pre_alert_minutes, 15);
    }

    #[test]
    fn test_per_prayer_pre_alert_overrides() {
        let toml_str = r#"
[pre_alert]
fajr = 20
sunrise = 0
"#;
        let config: NotificationConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.pre_alert.get_minutes("Fajr", config.pre_alert_minutes), 20);
        assert_eq!(config.pre_alert.get_minutes("Sunrise", config.pre_alert_minutes), 0);
        assert_eq!(config.pre_alert.get_minutes("Dhuhr", config.pre_alert_minutes), 15);
    }

    #[test]
    fn test_missing_notifications_section_uses_defaults() {
        let toml_str = r#"
[location]
lat = 21.0
lon = 39.0
"#;
        let config: AppConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.notifications.desktop, true);
        assert_eq!(config.notifications.bell, true);
        assert_eq!(config.notifications.pre_alert_minutes, 15);
    }

    #[test]
    fn test_appconfig_with_notifications_section() {
        let toml_str = r#"
[location]
lat = 21.0
lon = 39.0

[notifications]
desktop = true
bell = false
pre_alert_minutes = 10

[notifications.pre_alert]
fajr = 20
"#;
        let config: AppConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.notifications.desktop, true);
        assert_eq!(config.notifications.bell, false);
        assert_eq!(config.notifications.pre_alert_minutes, 10);
        assert_eq!(config.notifications.pre_alert.fajr, Some(20));
    }

    #[test]
    fn test_default_config_contains_notifications_section() {
        let dir = std::env::temp_dir().join("tui-adhan-test-notif");
        let path = dir.join("config.toml");
        generate_default_config(&path).unwrap();
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("[notifications]"), "default config should contain [notifications] section");
        assert!(content.contains("pre_alert_minutes"), "default config should mention pre_alert_minutes");
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn test_sunrise_defaults_to_zero_pre_alert() {
        let config: PreAlertConfig = Default::default();
        // Sunrise defaults to 0 (skip) unless explicitly set
        assert_eq!(config.get_minutes("Sunrise", 15), 0);
    }
}
