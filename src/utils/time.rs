use std::time::{SystemTime, UNIX_EPOCH};

const EPOCH_SECOND: i64 = 1;
const EPOCH_SECOND_MILLIS: i64 = 1000;
const EPOCH_MINUTE: i64 = EPOCH_SECOND * 60;
const EPOCH_MINUTE_MILLIS: i64 = EPOCH_SECOND_MILLIS * 60;
const EPOCH_HOUR: i64 = EPOCH_MINUTE * 60;
const EPOCH_HOUR_MILLIS: i64 = EPOCH_MINUTE_MILLIS * 60;

pub fn get_epoch_time() -> i64 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
}
