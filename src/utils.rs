use std::time::{SystemTime, UNIX_EPOCH};

pub fn unix_epoch_seconds() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is before Unix epoch")
        .as_secs() as i64
}
