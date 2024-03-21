use chrono::NaiveDate;

use crate::general::*;
use crate::market::chaintypes::Chat;
use crate::market::calendar::*;

pub struct TimeInfo {
    pub ts:Timestamp,
    pub date:NaiveDate,
    pub first_of_day:bool,
    pub last_of_day:bool,
    pub at_close:bool,
}

pub fn make_tinfo(date_prev:NaiveDate, ts:&Timestamp, ts_next:&Timestamp) -> TimeInfo {
    let date = ts.date();
    let at_close = ts.time() == market_close(date);
    let last_of_day = ts_next.date() != date;
    let first_of_day = date != date_prev;
    TimeInfo {
        ts: *ts,
        date,
        first_of_day,
        last_of_day,
        at_close,
    }
}

pub trait Strategy {
    fn run_for_ts<T:Chat>(&mut self, ts:&TimeInfo, chat:&T);
}

// fn run_range<F>(&self, rang:Range<Timestamp>, f:F)
// where F:FnMut(&TimeInfo, &Self::R) -> ();
