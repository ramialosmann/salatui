mod cli;
mod config;
mod prayer;

use clap::Parser;

use cli::Cli;
use config::{config_path, load_or_create_config, merge_config_with_cli};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = load_or_create_config()?;
    let merged = merge_config_with_cli(config, &cli);

    let (lat, lon) = match (merged.location.lat, merged.location.lon) {
        (Some(lat), Some(lon)) => (lat, lon),
        _ => {
            eprintln!("Error: latitude and longitude are required.");
            eprintln!(
                "Set them in your config file: {}",
                config_path()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|_| "~/.config/tui-adhan/config.toml".to_string())
            );
            eprintln!("Or use: tui-adhan --lat <LATITUDE> --lon <LONGITUDE>");
            std::process::exit(1);
        }
    };

    // Placeholder until prayer calculation is implemented in Plan 02
    println!(
        "tui-adhan: coordinates set to ({}, {}), method={}, madhab={}",
        lat, lon, merged.calculation.method, merged.calculation.madhab
    );
    println!("Prayer calculation will be implemented in the next plan.");

    Ok(())
}
