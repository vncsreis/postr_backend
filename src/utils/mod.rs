use chrono::{NaiveDateTime, Utc};

pub mod jwt;

pub fn current_time() -> NaiveDateTime {
    NaiveDateTime::from(Utc::now().naive_utc())
}
