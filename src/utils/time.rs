use chrono::offset::Utc;
use chrono::{DateTime, Duration};
use std::time::SystemTime;

pub fn current_time() -> String {
    let system_time = SystemTime::now();
    let datetime: DateTime<Utc> = system_time.into();
    datetime.format("%d/%m/%Y %T").to_string()
}

pub fn long_expiry_time(fmt: &str) -> String {
    let expiry: DateTime<Utc> = Utc::now() + Duration::days(365 * 50);
    expiry.format(fmt).to_string()
}
