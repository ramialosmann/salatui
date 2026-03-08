use salah::prelude::*;

pub struct PrayerResult {
    pub fajr: chrono::DateTime<chrono::Local>,
    pub sunrise: chrono::DateTime<chrono::Local>,
    pub dhuhr: chrono::DateTime<chrono::Local>,
    pub asr: chrono::DateTime<chrono::Local>,
    pub maghrib: chrono::DateTime<chrono::Local>,
    pub isha: chrono::DateTime<chrono::Local>,
}

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
        _ => anyhow::bail!(
            "Unknown calculation method: '{}'. Valid methods: mwl, egyptian, karachi, \
             umm_al_qura, dubai, moonsighting_committee, north_america (isna), kuwait, \
             qatar, singapore, tehran, turkey",
            s
        ),
    }
}

pub fn parse_madhab(s: &str) -> anyhow::Result<Madhab> {
    match s.to_lowercase().as_str() {
        "shafi" | "shafii" | "standard" => Ok(Madhab::Shafi),
        "hanafi" => Ok(Madhab::Hanafi),
        _ => anyhow::bail!("Unknown madhab: '{}'. Valid options: shafi, hanafi", s),
    }
}

/// Calculate prayer times for a specific date.
pub fn calculate_prayers_for_date(
    lat: f64,
    lon: f64,
    method: Method,
    madhab: Madhab,
    date: chrono::NaiveDate,
) -> anyhow::Result<PrayerResult> {
    let coords = Coordinates::new(lat, lon);
    // Configuration::with() returns Parameters with MiddleOfTheNight high-latitude rule
    // by default. This safely prevents NaN/panic at all latitudes.
    // Note: HighLatitudeRule::recommended() would use SeventhOfTheNight for lat > 48,
    // but the type is not publicly re-exported by the salah crate (module is private).
    // MiddleOfTheNight is a safe conservative fallback for all latitudes.
    let params = Configuration::with(method, madhab);

    let prayers = PrayerSchedule::new()
        .on(date)
        .for_location(coords)
        .with_configuration(params)
        .calculate()
        .map_err(|e| anyhow::anyhow!("Failed to calculate prayer times: {}", e))?;

    Ok(PrayerResult {
        fajr: prayers.time(Prayer::Fajr).with_timezone(&chrono::Local),
        sunrise: prayers.time(Prayer::Sunrise).with_timezone(&chrono::Local),
        dhuhr: prayers.time(Prayer::Dhuhr).with_timezone(&chrono::Local),
        asr: prayers.time(Prayer::Asr).with_timezone(&chrono::Local),
        maghrib: prayers.time(Prayer::Maghrib).with_timezone(&chrono::Local),
        isha: prayers.time(Prayer::Isha).with_timezone(&chrono::Local),
    })
}

/// Calculate prayer times for today.
pub fn calculate_prayers(
    lat: f64,
    lon: f64,
    method: Method,
    madhab: Madhab,
) -> anyhow::Result<PrayerResult> {
    let date = chrono::Local::now().date_naive();
    calculate_prayers_for_date(lat, lon, method, madhab, date)
}

pub fn print_prayers(prayers: &PrayerResult, time_format: &str) {
    let fmt = match time_format {
        "12h" => "%I:%M %p",
        _ => "%H:%M",
    };

    let times = [
        ("Fajr", &prayers.fajr),
        ("Sunrise", &prayers.sunrise),
        ("Dhuhr", &prayers.dhuhr),
        ("Asr", &prayers.asr),
        ("Maghrib", &prayers.maghrib),
        ("Isha", &prayers.isha),
    ];

    for (name, time) in &times {
        println!("{:<10} {}", name, time.format(fmt));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculates_all_six_prayers() {
        let method = parse_method("mwl").unwrap();
        let madhab = parse_madhab("shafi").unwrap();
        let result = calculate_prayers(21.4225, 39.8262, method, madhab);
        assert!(result.is_ok(), "calculate_prayers failed: {:?}", result.err());

        let prayers = result.unwrap();
        // Verify chronological order: Fajr < Sunrise < Dhuhr < Asr < Maghrib < Isha
        assert!(prayers.fajr < prayers.sunrise, "Fajr should be before Sunrise");
        assert!(prayers.sunrise < prayers.dhuhr, "Sunrise should be before Dhuhr");
        assert!(prayers.dhuhr < prayers.asr, "Dhuhr should be before Asr");
        assert!(prayers.asr < prayers.maghrib, "Asr should be before Maghrib");
        assert!(prayers.maghrib < prayers.isha, "Maghrib should be before Isha");
    }

    #[test]
    fn test_different_methods() {
        let madhab = parse_madhab("shafi").unwrap();

        let mwl = calculate_prayers(21.4225, 39.8262, parse_method("mwl").unwrap(), madhab).unwrap();
        let egyptian = calculate_prayers(21.4225, 39.8262, parse_method("egyptian").unwrap(), madhab).unwrap();

        // MWL uses 18.0 Fajr angle, Egyptian uses 19.5 -- different Fajr times
        assert_ne!(
            mwl.fajr.format("%H:%M").to_string(),
            egyptian.fajr.format("%H:%M").to_string(),
            "MWL and Egyptian should produce different Fajr times"
        );
    }

    #[test]
    fn test_madhab_affects_asr() {
        let method = parse_method("mwl").unwrap();

        let shafi = calculate_prayers(21.4225, 39.8262, method, parse_madhab("shafi").unwrap()).unwrap();
        let hanafi = calculate_prayers(21.4225, 39.8262, method, parse_madhab("hanafi").unwrap()).unwrap();

        // Hanafi Asr is later (2x shadow ratio vs 1x)
        assert!(
            hanafi.asr > shafi.asr,
            "Hanafi Asr ({}) should be later than Shafi Asr ({})",
            hanafi.asr.format("%H:%M"),
            shafi.asr.format("%H:%M")
        );
    }

    #[test]
    fn test_high_latitude_no_panic() {
        let method = parse_method("mwl").unwrap();
        let madhab = parse_madhab("shafi").unwrap();
        let result = calculate_prayers(59.9139, 10.7522, method, madhab);
        assert!(result.is_ok(), "Oslo calculation failed: {:?}", result.err());

        let prayers = result.unwrap();
        // Verify times are valid (not NaN equivalent -- check they can be formatted)
        let _ = prayers.fajr.format("%H:%M").to_string();
        let _ = prayers.isha.format("%H:%M").to_string();
    }

    #[test]
    fn test_parse_method_valid() {
        assert_eq!(parse_method("mwl").unwrap(), Method::MuslimWorldLeague);
        assert_eq!(parse_method("isna").unwrap(), Method::NorthAmerica);
        assert_eq!(parse_method("egyptian").unwrap(), Method::Egyptian);
        assert_eq!(parse_method("MWL").unwrap(), Method::MuslimWorldLeague); // case insensitive
    }

    #[test]
    fn test_parse_method_invalid() {
        assert!(parse_method("invalid_method").is_err());
    }

    #[test]
    fn test_parse_madhab_valid() {
        assert_eq!(parse_madhab("shafi").unwrap(), Madhab::Shafi);
        assert_eq!(parse_madhab("hanafi").unwrap(), Madhab::Hanafi);
        assert_eq!(parse_madhab("shafii").unwrap(), Madhab::Shafi);
        assert_eq!(parse_madhab("standard").unwrap(), Madhab::Shafi);
    }

    #[test]
    fn test_parse_madhab_invalid() {
        assert!(parse_madhab("maliki").is_err());
    }

    #[test]
    fn test_calculate_prayers_for_date() {
        let method = parse_method("mwl").unwrap();
        let madhab = parse_madhab("shafi").unwrap();
        let date = chrono::NaiveDate::from_ymd_opt(2026, 3, 9).unwrap();
        let result = calculate_prayers_for_date(21.4225, 39.8262, method, madhab, date);
        assert!(
            result.is_ok(),
            "calculate_prayers_for_date failed: {:?}",
            result.err()
        );

        let prayers = result.unwrap();
        // Verify chronological order
        assert!(prayers.fajr < prayers.sunrise);
        assert!(prayers.sunrise < prayers.dhuhr);
        assert!(prayers.dhuhr < prayers.asr);
        assert!(prayers.asr < prayers.maghrib);
        assert!(prayers.maghrib < prayers.isha);
    }
}
