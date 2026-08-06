pub fn cron_due(expression: &str, now: chrono::DateTime<chrono::Utc>) -> bool {
    use chrono::{Datelike, Timelike};
    let fields = expression.split_whitespace().collect::<Vec<_>>();
    fields.len() == 5
        && matches_field(fields[0], now.minute())
        && matches_field(fields[1], now.hour())
        && matches_field(fields[2], now.day())
        && matches_field(fields[3], now.month())
        && matches_field(fields[4], now.weekday().num_days_from_sunday())
}

fn matches_field(field: &str, value: u32) -> bool {
    field.split(',').any(|item| {
        if item == "*" {
            true
        } else if let Some(step) = item.strip_prefix("*/").and_then(|v| v.parse::<u32>().ok()) {
            step > 0 && value % step == 0
        } else {
            item.parse::<u32>().ok() == Some(value)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::cron_due;
    use chrono::TimeZone;
    #[test]
    fn matches_exact_step_and_wildcard_cron_fields() {
        let now = chrono::Utc.with_ymd_and_hms(2026, 8, 7, 3, 30, 0).unwrap();
        assert!(cron_due("30 3 * * *", now));
        assert!(cron_due("*/15 * * * *", now));
        assert!(!cron_due("0 4 * * *", now));
    }
}
