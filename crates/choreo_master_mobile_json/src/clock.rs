use time::OffsetDateTime;

pub(crate) struct SystemClock;

impl SystemClock {
    pub(crate) fn now_utc() -> OffsetDateTime {
        #[cfg(target_arch = "wasm32")]
        {
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
