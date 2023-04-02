use std::ops::Range;

use crate::general::*;
use crate::market::types::*;
use crate::market::chaintypes::{Chall, Chat};

pub fn backtest<T:Chall>(chall:&T, rang:&Range<Timestamp>) {
    chall.run_range(rang.clone(), handle_ts);
}

pub trait Strategy {
    fn run_for_ts<T:Chat>(&mut self, ts:Timestamp, chat:T);
}

pub struct Strat1 {
    under_prev: PriceCalc,
}

impl Strategy for Strat1 {
    fn run_for_ts<T:Chat>(&mut self, ts:Timestamp, chat:T) {
        let under = chat.under();
        if (under - self.under_prev).abs() > 1.0 {
            self.under_prev = chat.under();
            println!("Found under move {under}");
        }
    }
}

fn handle_ts<T:Chat>(ts:&Timestamp, _chat:&T) {
    println!("{}", ts);
}

// fn handle_ts<T>(ts:&Timestamp, chat:&T)
//     where T:Chall,
//         <T as Chall>::R: Chat {
//     // chat
// }