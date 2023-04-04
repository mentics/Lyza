use chrono::NaiveDate;
use lazy_static::lazy_static;
use std::ops::Range;
use std::sync::Mutex;

use crate::general::*;
use crate::market::calendar;
use crate::market::chaintypes::{Chall, ChainsAll};
use super::strategy::{Strategy, TimeInfo, make_tinfo};

use super::strat1::make_strat1;

lazy_static! {
    static ref CHALL_CACHE: Mutex<ChainsAll> = {
        Mutex::new(crate::store::histdata::load_chall("m1"))
    };
}

pub fn check_cal() {
    let close = calendar::market_close(NaiveDate::from_ymd_opt(2021,2,3).unwrap());
    println!("{close}");
}

pub fn run() {
    let chall = &*CHALL_CACHE.lock().unwrap();

    let range_m1 = (Timestamp::from_ymd(2022, 1, 1))..(Timestamp::from_ymd(2022, 2, 1));
    let mut strat = make_strat1();

    backtest(chall, &range_m1, &mut strat);
}

pub fn backtest<T:Chall, S:Strategy>(chall:&T, rang:&Range<Timestamp>, strat:&mut S) {
    let mut itr = chall.range(rang);
    run_range(&mut itr,
        |ts, chat| strat.run_for_ts(ts, chat)
    );
}

fn run_range<'a, R, I:Iterator<Item=(&'a Timestamp, R)>, F:FnMut(&TimeInfo, R) -> ()>(iter:&mut I, mut f:F) {
    let mut entry = iter.next().unwrap();
    let mut date_prev = NaiveDate::MIN;
    for next in iter {
        let (ts, chat) = entry;
        let tinfo = make_tinfo(date_prev, ts, next.0);
        f(&tinfo, chat);
        entry = next;
        date_prev = tinfo.date;
    }
}

// fn run_range<C,I,R,F>(iter:I, mut f:F)
// where
//         C:Chat,
//         R:(Timestamp,C),
//         I:Iterator<Item=R>,
//         F:FnMut(&TimeInfo, R) -> () {
//     let mut entry = iter.next().unwrap();
//     let mut date_prev = NaiveDate::MIN;
//     for next in iter {
//         let (ts, chat) = entry;
//         let tinfo = make_tinfo(date_prev, ts, next.0);
//         f(&tinfo, chat);
//         entry = next;
//         date_prev = tinfo.date;
//     }
// }


