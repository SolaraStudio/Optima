use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

pub fn current_timestamp_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub fn current_timestamp_nanos() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64
}

pub fn to_rfc3339(timestamp_secs: u64) -> String {
    let dt = chrono::DateTime::<chrono::Utc>::from_timestamp(timestamp_secs as i64, 0);
    if let Some(dt) = dt {
        dt.to_rfc3339()
    } else {
        String::new()
    }
}

pub fn from_rfc3339(s: &str) -> Option<u64> {
    chrono::DateTime::parse_from_rfc3339(s)
        .ok()
        .map(|dt| dt.timestamp() as u64)
}

pub fn format_time(ms: u64) -> String {
    let seconds = ms / 1000;
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let remaining_seconds = seconds % 60;
    let remaining_minutes = minutes % 60;

    if hours > 0 {
        format!(
            "{:02}:{:02}:{:02}",
            hours, remaining_minutes, remaining_seconds
        )
    } else {
        format!("{:02}:{:02}", remaining_minutes, remaining_seconds)
    }
}

pub fn parse_time(s: &str) -> Option<u64> {
    let parts: Vec<&str> = s.split(':').collect();
    match parts.len() {
        2 => {
            let minutes: u64 = parts[0].parse().ok()?;
            let seconds: u64 = parts[1].parse().ok()?;
            Some(minutes * 60 * 1000 + seconds * 1000)
        }
        3 => {
            let hours: u64 = parts[0].parse().ok()?;
            let minutes: u64 = parts[1].parse().ok()?;
            let seconds: u64 = parts[2].parse().ok()?;
            Some(hours * 60 * 60 * 1000 + minutes * 60 * 1000 + seconds * 1000)
        }
        _ => None,
    }
}
