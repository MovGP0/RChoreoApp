use time::OffsetDateTime;

pub struct SystemClock;

impl SystemClock {
    pub fn now_utc() -> OffsetDateTime {
        #[cfg(target_arch = "wasm32")]
        {
            // In browsers, Performance timestamps are monotonic and provide a unix-based origin.
            let millis_since_unix_epoch = web_sys::window()
                .and_then(|window| window.performance())
                .map(|performance| performance.time_origin() + performance.now())
                .unwrap_or(0.0);
            let nanos = (millis_since_unix_epoch * 1_000_000.0) as i128;
            OffsetDateTime::from_unix_timestamp_nanos(nanos).unwrap_or(OffsetDateTime::UNIX_EPOCH)
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            OffsetDateTime::now_utc()
        }
    }
}

pub(crate) fn parse_timestamp_seconds(value: &str) -> Option<f64> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }

    let mut parts = value.split(':').collect::<Vec<_>>();
    if parts.len() > 3 {
        return None;
    }

    let seconds_part = parts.pop()?;
    let minutes_part = parts.pop().unwrap_or("0");
    let hours_part = parts.pop().unwrap_or("0");

    let seconds = seconds_part.parse::<f64>().ok()?;
    let minutes = minutes_part.parse::<f64>().ok()?;
    let hours = hours_part.parse::<f64>().ok()?;

    Some(hours * 3600.0 + minutes * 60.0 + seconds)
}

pub(crate) fn format_seconds(value: f64) -> String {
    let mut text = format!("{value:.3}");
    if text.find('.').is_some() {
        while text.ends_with('0') {
            text.pop();
        }
        if text.ends_with('.') {
            text.pop();
        }
        if text.is_empty() {
            text.push('0');
        }
    }
    text
}
