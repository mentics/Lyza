use chrono::prelude::{NaiveDate,NaiveDateTime};

pub type Timestamp = NaiveDateTime;
pub type ExpirDate = NaiveDate;

pub static TS_ZERO: Timestamp = NaiveDateTime::MIN;
