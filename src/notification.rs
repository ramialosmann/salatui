use std::collections::HashSet;

use chrono::{DateTime, Local};

use crate::config::NotificationConfig;

/// Tracks which prayer notifications have already been sent to prevent duplicates.
pub struct NotificationTracker {
    notified: HashSet<String>,
}

impl NotificationTracker {
    pub fn new() -> Self {
        Self {
            notified: HashSet::new(),
        }
    }

    pub fn is_notified(&self, key: &str) -> bool {
        self.notified.contains(key)
    }

    pub fn mark_notified(&mut self, key: &str) {
        self.notified.insert(key.to_string());
    }

    pub fn reset(&mut self) {
        self.notified.clear();
    }
}

/// Represents a notification action to be executed.
pub enum NotificationAction {
    AtTime { prayer: String, time_str: String },
    PreAlert { prayer: String, minutes: u32 },
}

/// Checks all prayers and returns notification actions that should fire now.
pub fn check_notifications(
    prayer_list: &[(&str, DateTime<Local>)],
    config: &NotificationConfig,
    tracker: &NotificationTracker,
    now: DateTime<Local>,
    time_format: &str,
) -> Vec<NotificationAction> {
    let mut actions = Vec::new();

    for &(name, time) in prayer_list {
        let pre_alert_minutes = config.pre_alert.get_minutes(name, config.pre_alert_minutes);

        // At-time check: skip Sunrise by default (pre_alert_minutes == 0 means sunrise is skipped,
        // but we also need to skip at-time for Sunrise unless explicitly overridden)
        let skip_at_time = name == "Sunrise" && config.pre_alert.sunrise.is_none();

        if !skip_at_time {
            let at_key = format!("{}_at", name.to_lowercase());
            if !tracker.is_notified(&at_key)
                && now >= time
                && now < time + chrono::Duration::seconds(60)
            {
                let time_str = format_notification_body(time, time_format);
                actions.push(NotificationAction::AtTime {
                    prayer: name.to_string(),
                    time_str,
                });
            }
        }

        // Pre-alert check
        if pre_alert_minutes > 0 {
            let pre_key = format!("{}_pre", name.to_lowercase());
            let pre_alert_start = time - chrono::Duration::minutes(pre_alert_minutes as i64);
            if !tracker.is_notified(&pre_key) && now >= pre_alert_start && now < time {
                actions.push(NotificationAction::PreAlert {
                    prayer: name.to_string(),
                    minutes: pre_alert_minutes,
                });
            }
        }
    }

    actions
}

/// Executes a notification action (sends desktop/bell, marks tracker).
pub fn execute_action(
    action: &NotificationAction,
    config: &NotificationConfig,
    tracker: &mut NotificationTracker,
) {
    match action {
        NotificationAction::AtTime { prayer, time_str } => {
            if config.desktop {
                send_desktop(prayer, time_str);
            }
            if config.bell {
                send_bell();
            }
            tracker.mark_notified(&format!("{}_at", prayer.to_lowercase()));
        }
        NotificationAction::PreAlert { prayer, minutes } => {
            if config.desktop {
                let body = format_pre_alert_body(prayer, *minutes);
                send_desktop(prayer, &body);
            }
            tracker.mark_notified(&format!("{}_pre", prayer.to_lowercase()));
        }
    }
}

/// Sends a desktop notification via notify-send.
pub fn send_desktop(title: &str, body: &str) {
    let _ = std::process::Command::new("notify-send")
        .arg(title)
        .arg(body)
        .spawn();
}

/// Sends terminal bell (BEL character).
pub fn send_bell() {
    print!("\x07");
}

/// Checks if notify-send is available on the system.
pub fn check_notify_send_available() -> bool {
    std::process::Command::new("which")
        .arg("notify-send")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Formats the notification body for at-time notifications.
/// Includes mosque emoji prefix.
pub fn format_notification_body(time: DateTime<Local>, time_format: &str) -> String {
    let formatted = match time_format {
        "12h" => time.format("%-I:%M %p").to_string(),
        _ => time.format("%H:%M").to_string(),
    };
    format!("\u{1f54c} Prayer time: {}", formatted)
}

/// Formats the pre-alert notification body (no emoji).
pub fn format_pre_alert_body(prayer: &str, minutes: u32) -> String {
    format!("{} in {} minutes", prayer, minutes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn make_config(desktop: bool, bell: bool, pre_alert_minutes: u32) -> NotificationConfig {
        NotificationConfig {
            desktop,
            bell,
            pre_alert_minutes,
            pre_alert: Default::default(),
        }
    }

    #[test]
    fn test_tracker_starts_empty() {
        let tracker = NotificationTracker::new();
        assert!(!tracker.is_notified("fajr_at"));
    }

    #[test]
    fn test_tracker_records_and_prevents_duplicates() {
        let mut tracker = NotificationTracker::new();
        tracker.mark_notified("fajr_at");
        assert!(tracker.is_notified("fajr_at"));
        assert!(!tracker.is_notified("dhuhr_at"));
    }

    #[test]
    fn test_tracker_reset_clears_all() {
        let mut tracker = NotificationTracker::new();
        tracker.mark_notified("fajr_at");
        tracker.mark_notified("dhuhr_pre");
        tracker.reset();
        assert!(!tracker.is_notified("fajr_at"));
        assert!(!tracker.is_notified("dhuhr_pre"));
    }

    #[test]
    fn test_check_notifications_at_time() {
        let config = make_config(true, true, 15);
        let tracker = NotificationTracker::new();
        let now = Local.with_ymd_and_hms(2026, 3, 9, 12, 0, 0).unwrap();
        let prayer_list: Vec<(&str, DateTime<Local>)> = vec![
            ("Dhuhr", now),
        ];

        let actions = check_notifications(&prayer_list, &config, &tracker, now, "24h");
        assert_eq!(actions.len(), 1); // only at-time, pre-alert requires now < time
        assert!(matches!(&actions[0], NotificationAction::AtTime { prayer, .. } if prayer == "Dhuhr"));
    }

    #[test]
    fn test_check_notifications_pre_alert() {
        let config = make_config(true, true, 15);
        let tracker = NotificationTracker::new();
        let prayer_time = Local.with_ymd_and_hms(2026, 3, 9, 12, 0, 0).unwrap();
        let now = prayer_time - chrono::Duration::minutes(10);
        let prayer_list: Vec<(&str, DateTime<Local>)> = vec![
            ("Dhuhr", prayer_time),
        ];

        let actions = check_notifications(&prayer_list, &config, &tracker, now, "24h");
        assert_eq!(actions.len(), 1);
        assert!(matches!(&actions[0], NotificationAction::PreAlert { prayer, minutes } if prayer == "Dhuhr" && *minutes == 15));
    }

    #[test]
    fn test_sunrise_skipped_at_time_by_default() {
        let config = make_config(true, true, 15);
        let tracker = NotificationTracker::new();
        let now = Local.with_ymd_and_hms(2026, 3, 9, 6, 30, 0).unwrap();
        let prayer_list: Vec<(&str, DateTime<Local>)> = vec![
            ("Sunrise", now),
        ];

        let actions = check_notifications(&prayer_list, &config, &tracker, now, "24h");
        // Sunrise at-time should be skipped, and sunrise pre_alert defaults to 0 so no pre-alert either
        assert_eq!(actions.len(), 0);
    }

    #[test]
    fn test_already_notified_prevents_duplicate() {
        let config = make_config(true, true, 15);
        let mut tracker = NotificationTracker::new();
        tracker.mark_notified("dhuhr_at");
        let now = Local.with_ymd_and_hms(2026, 3, 9, 12, 0, 0).unwrap();
        let prayer_list: Vec<(&str, DateTime<Local>)> = vec![
            ("Dhuhr", now),
        ];

        let actions = check_notifications(&prayer_list, &config, &tracker, now, "24h");
        assert_eq!(actions.len(), 0);
    }

    #[test]
    fn test_format_notification_body_24h() {
        let time = Local.with_ymd_and_hms(2026, 3, 9, 18, 23, 0).unwrap();
        let body = format_notification_body(time, "24h");
        assert_eq!(body, "\u{1f54c} Prayer time: 18:23");
    }

    #[test]
    fn test_format_notification_body_12h() {
        let time = Local.with_ymd_and_hms(2026, 3, 9, 18, 23, 0).unwrap();
        let body = format_notification_body(time, "12h");
        assert_eq!(body, "\u{1f54c} Prayer time: 6:23 PM");
    }

    #[test]
    fn test_format_pre_alert_body() {
        let body = format_pre_alert_body("Maghrib", 15);
        assert_eq!(body, "Maghrib in 15 minutes");
    }
}
