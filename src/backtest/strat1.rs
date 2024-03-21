use crate::market::{types::*, chaintypes::Chat};
use super::strategy::*;

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
    fn run_for_ts<T:Chat>(&mut self, tinfo:&TimeInfo, chat:&T) {
        let under = chat.under();
        let moved = (under - self.under_prev).abs() > 1.0;
        if moved {
            self.under_prev = chat.under();
        }
        if moved || tinfo.first_of_day || tinfo.last_of_day {
            self.check_exit(tinfo, chat);
        }
        if moved {
            self.check_entry(tinfo, chat);
        }
    }
}

impl Strat1 {
    fn check_exit<T:Chat>(&mut self, tinfo:&TimeInfo, chat:&T) {
        println!("check_exit {}", chat.under());
    }

    fn check_entry<T:Chat>(&mut self, tinfo:&TimeInfo, chat:&T) {
        println!("check_entry {}", chat.under());
    }
}