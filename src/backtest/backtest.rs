use lazy_static::lazy_static;
use std::ops::Range;
use std::sync::Mutex;

use crate::general::*;
use crate::market::types::*;
use crate::market::chaintypes::{Chall, Chat, ChainsAll};

lazy_static! {
    static ref CHALL_CACHE: Mutex<Option<ChainsAll>> = {
        Mutex::new(None)
    };
}


pub fn run() {
    let mut ch = CHALL_CACHE.lock().unwrap();
    if ch.is_none() {
        *ch = Some(crate::store::histdata::load_chall("m1"));
    }
    let chall = ch.as_ref().unwrap();

    let range_m1 = (Timestamp::from_ymd(2022, 1, 1))..(Timestamp::from_ymd(2022, 2, 1));
    let mut strat = make_strat1();

    backtest(chall, &range_m1, &mut strat);
}

pub fn backtest<T:Chall, S:Strategy>(chall:&T, rang:&Range<Timestamp>, strat:&mut S) {
    chall.run_range(rang.clone(),
        |ts, chat| strat.run_for_ts(ts, chat)
    );
}

pub trait Strategy {
    fn run_for_ts<T:Chat>(&mut self, ts:&Timestamp, chat:&T);
}

pub fn make_strat1() -> Strat1 {
    Strat1::new()
}

pub struct Strat1 {
    under_prev: PriceCalc,
}
impl Strat1 {
    fn new() -> Strat1 {
        Strat1 { under_prev: 0.0 }
    }
}

impl Strategy for Strat1 {
    fn run_for_ts<T:Chat>(&mut self, ts:&Timestamp, chat:&T) {
        let under = chat.under();
        if (under - self.under_prev).abs() > 1.0 {
            self.under_prev = chat.under();
            println!("Found under move {under}");
        }
    }
}

// fn handle_ts<T:Chat>(ts:&Timestamp, _chat:&T) {
//     println!("{}", ts);
// }

// fn handle_ts<T>(ts:&Timestamp, chat:&T)
//     where T:Chall,
//         <T as Chall>::R: Chat {
//     // chat
// }