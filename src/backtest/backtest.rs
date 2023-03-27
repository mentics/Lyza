use crate::general::*;
use crate::market::chaintypes::{Chall, Chat};

pub fn backtest<T:Chall>(chall:&T) {
    chall.run_all(handle_ts);
}

fn handle_ts<T:Chat>(ts:&Timestamp, _chat:&T) {
    println!("{}", ts);
}


// fn handle_ts<T>(ts:&Timestamp, chat:&T)
//     where T:Chall,
//         <T as Chall>::R: Chat {
//     // chat
// }