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
}
