use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "tui-adhan", about = "Islamic prayer times in your terminal")]
pub struct Cli {
    /// Latitude (overrides config)
    #[arg(long)]
    pub lat: Option<f64>,
    /// Longitude (overrides config)
    #[arg(long)]
    pub lon: Option<f64>,
    /// Calculation method (overrides config). Options: mwl, egyptian, karachi, umm_al_qura, dubai, moonsighting_committee, north_america, isna, kuwait, qatar, singapore, tehran, turkey
    #[arg(long)]
    pub method: Option<String>,
    /// Asr madhab: shafi or hanafi (overrides config)
    #[arg(long)]
    pub madhab: Option<String>,
}
