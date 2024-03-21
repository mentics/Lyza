use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref DATA: Mutex<Vec<i64>> = {
        Mutex::new(init())
    };
}

fn init() -> Vec<i64> {
    println!("Init DATA");
    return vec![1,2,3,4,5];
}

pub fn test() {
    println!("{}", DATA.lock().unwrap()[3]);
}