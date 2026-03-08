use chrono::{Datelike, Local};

use crate::prayer::{self, PrayerResult};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum View {
    Clock,
    Schedule,
}

pub struct App {
    pub running: bool,
    pub view: View,
    pub prayers: PrayerResult,
    pub tomorrow_fajr: Option<chrono::DateTime<Local>>,
    pub time_format: String,
    pub prayer_date: chrono::NaiveDate,
    lat: f64,
    lon: f64,
    method_str: String,
    madhab_str: String,
}

impl App {
    pub fn new(
        prayers: PrayerResult,
        time_format: String,
        lat: f64,
        lon: f64,
        method_str: String,
        madhab_str: String,
    ) -> Self {
        let prayer_date = Local::now().date_naive();
        let tomorrow_fajr = Self::compute_tomorrow_fajr(lat, lon, &method_str, &madhab_str, prayer_date);

        Self {
            running: true,
            view: View::Clock,
            prayers,
            tomorrow_fajr,
            time_format,
            prayer_date,
            lat,
            lon,
            method_str,
            madhab_str,
        }
    }

    fn compute_tomorrow_fajr(
        lat: f64,
        lon: f64,
        method_str: &str,
        madhab_str: &str,
        today: chrono::NaiveDate,
    ) -> Option<chrono::DateTime<Local>> {
        let tomorrow = today.succ_opt()?;
        let method = prayer::parse_method(method_str).ok()?;
        let madhab = prayer::parse_madhab(madhab_str).ok()?;
        let result = prayer::calculate_prayers_for_date(lat, lon, method, madhab, tomorrow).ok()?;
        Some(result.fajr)
    }

    /// Returns the next upcoming prayer name and time.
    /// After Isha, returns tomorrow's Fajr if available.
    pub fn next_prayer(&self) -> Option<(&str, chrono::DateTime<Local>)> {
        let now = Local::now();
        let prayers = [
            ("Fajr", self.prayers.fajr),
            ("Sunrise", self.prayers.sunrise),
            ("Dhuhr", self.prayers.dhuhr),
            ("Asr", self.prayers.asr),
            ("Maghrib", self.prayers.maghrib),
            ("Isha", self.prayers.isha),
        ];

        for (name, time) in &prayers {
            if *time > now {
                return Some((name, *time));
            }
        }

        // After Isha -- return tomorrow's Fajr
        self.tomorrow_fajr.map(|t| ("Fajr", t))
    }

    /// Formats countdown as "PrayerName in H:MM:SS".
    pub fn format_countdown(&self) -> String {
        match self.next_prayer() {
            Some((name, time)) => {
                let now = Local::now();
                let remaining = time - now;
                let total_secs = remaining.num_seconds();
                if total_secs < 0 {
                    return String::new();
                }
                let h = total_secs / 3600;
                let m = (total_secs % 3600) / 60;
                let s = total_secs % 60;
                format!("{} in {}:{:02}:{:02}", name, h, m, s)
            }
            None => String::new(),
        }
    }

    /// Formats the date line as "DD MonthName YYYY AH · DD Mon YYYY".
    pub fn format_date_line() -> String {
        let now = Local::now();
        let hd = hijri_date::HijriDate::from_gr(
            now.year() as usize,
            now.month() as usize,
            now.day() as usize,
        );

        match hd {
            Ok(hd) => {
                let hijri_month = match hd.month() {
                    1 => "Muharram",
                    2 => "Safar",
                    3 => "Rabi al-Awwal",
                    4 => "Rabi al-Thani",
                    5 => "Jumada al-Ula",
                    6 => "Jumada al-Thani",
                    7 => "Rajab",
                    8 => "Sha'ban",
                    9 => "Ramadan",
                    10 => "Shawwal",
                    11 => "Dhul Qi'dah",
                    12 => "Dhul Hijjah",
                    _ => "Unknown",
                };
                let hijri = format!("{} {} {} AH", hd.day(), hijri_month, hd.year());
                let greg = now.format("%-d %b %Y").to_string();
                format!("{} \u{00B7} {}", hijri, greg)
            }
            Err(_) => {
                // Fallback to Gregorian only if Hijri conversion fails
                now.format("%-d %b %Y").to_string()
            }
        }
    }

    /// Toggle between Clock and Schedule views.
    pub fn toggle_view(&mut self) {
        self.view = match self.view {
            View::Clock => View::Schedule,
            View::Schedule => View::Clock,
        };
    }

    /// Called every tick -- recalculates prayers if the date has changed (midnight crossing).
    pub fn tick(&mut self) {
        let today = Local::now().date_naive();
        if today != self.prayer_date {
            if let (Ok(method), Ok(madhab)) = (
                prayer::parse_method(&self.method_str),
                prayer::parse_madhab(&self.madhab_str),
            ) {
                if let Ok(new_prayers) =
                    prayer::calculate_prayers_for_date(self.lat, self.lon, method, madhab, today)
                {
                    self.prayers = new_prayers;
                    self.prayer_date = today;
                    self.tomorrow_fajr =
                        Self::compute_tomorrow_fajr(self.lat, self.lon, &self.method_str, &self.madhab_str, today);
                }
            }
        }
    }

    /// Returns array of (name, time) tuples for all 6 prayers.
    pub fn prayer_list(&self) -> [(&str, chrono::DateTime<Local>); 6] {
        [
            ("Fajr", self.prayers.fajr),
            ("Sunrise", self.prayers.sunrise),
            ("Dhuhr", self.prayers.dhuhr),
            ("Asr", self.prayers.asr),
            ("Maghrib", self.prayers.maghrib),
            ("Isha", self.prayers.isha),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    /// Creates a PrayerResult with times relative to `base`.
    /// Fajr=base-5h, Sunrise=base-3h, Dhuhr=base+1h, Asr=base+4h, Maghrib=base+7h, Isha=base+8h
    fn make_prayers_relative(base: chrono::DateTime<Local>) -> PrayerResult {
        PrayerResult {
            fajr: base - Duration::hours(5),
            sunrise: base - Duration::hours(3),
            dhuhr: base + Duration::hours(1),
            asr: base + Duration::hours(4),
            maghrib: base + Duration::hours(7),
            isha: base + Duration::hours(8),
        }
    }

    /// Creates a PrayerResult where all prayers are in the past relative to now.
    fn make_all_past_prayers() -> PrayerResult {
        let now = Local::now();
        PrayerResult {
            fajr: now - Duration::hours(12),
            sunrise: now - Duration::hours(10),
            dhuhr: now - Duration::hours(6),
            asr: now - Duration::hours(4),
            maghrib: now - Duration::hours(2),
            isha: now - Duration::hours(1),
        }
    }

    fn make_test_app(prayers: PrayerResult) -> App {
        App {
            running: true,
            view: View::Clock,
            prayers,
            tomorrow_fajr: Some(Local::now() + Duration::hours(10)),
            time_format: "24h".to_string(),
            prayer_date: Local::now().date_naive(),
            lat: 21.4225,
            lon: 39.8262,
            method_str: "mwl".to_string(),
            madhab_str: "shafi".to_string(),
        }
    }

    #[test]
    fn test_next_prayer_returns_future_prayer() {
        let now = Local::now();
        let prayers = make_prayers_relative(now);
        let app = make_test_app(prayers);

        let next = app.next_prayer();
        assert!(next.is_some());
        let (name, time) = next.unwrap();
        // Dhuhr is the first future prayer (now+1h)
        assert_eq!(name, "Dhuhr");
        assert!(time > now);
    }

    #[test]
    fn test_next_prayer_after_isha_returns_tomorrow_fajr() {
        let prayers = make_all_past_prayers();
        let tomorrow_fajr = Local::now() + Duration::hours(10);
        let mut app = make_test_app(prayers);
        app.tomorrow_fajr = Some(tomorrow_fajr);

        let next = app.next_prayer();
        assert!(next.is_some());
        let (name, _time) = next.unwrap();
        assert_eq!(name, "Fajr");
    }

    #[test]
    fn test_toggle_view() {
        let now = Local::now();
        let prayers = make_prayers_relative(now);
        let mut app = make_test_app(prayers);

        assert_eq!(app.view, View::Clock);
        app.toggle_view();
        assert_eq!(app.view, View::Schedule);
        app.toggle_view();
        assert_eq!(app.view, View::Clock);
    }

    #[test]
    fn test_format_date_line() {
        let line = App::format_date_line();
        assert!(line.contains("AH"), "Date line should contain 'AH': {}", line);
        assert!(
            line.contains('\u{00B7}'),
            "Date line should contain middle dot: {}",
            line
        );
    }

    #[test]
    fn test_countdown_format() {
        let now = Local::now();
        let prayers = make_prayers_relative(now);
        let app = make_test_app(prayers);

        let countdown = app.format_countdown();
        // Should match "PrayerName in H:MM:SS" pattern
        assert!(
            countdown.contains(" in "),
            "Countdown should contain ' in ': {}",
            countdown
        );
        // Should contain a colon-separated time
        let parts: Vec<&str> = countdown.split(" in ").collect();
        assert_eq!(parts.len(), 2);
        let time_parts: Vec<&str> = parts[1].split(':').collect();
        assert_eq!(time_parts.len(), 3, "Time should be H:MM:SS format");
    }

    #[test]
    fn test_prayer_list_returns_six() {
        let now = Local::now();
        let prayers = make_prayers_relative(now);
        let app = make_test_app(prayers);

        let list = app.prayer_list();
        assert_eq!(list.len(), 6);
        assert_eq!(list[0].0, "Fajr");
        assert_eq!(list[5].0, "Isha");
    }

    #[test]
    fn test_app_starts_running_clock_view() {
        let now = Local::now();
        let prayers = make_prayers_relative(now);
        let app = make_test_app(prayers);

        assert!(app.running);
        assert_eq!(app.view, View::Clock);
    }
}
