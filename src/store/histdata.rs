use chrono::NaiveDateTime;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use crate::market::types::*;
use crate::store::loader;
use rkyv::{self, Deserialize, Archive};

pub struct HistData {
    // tss: Vec<NaiveDateTime>,
    calls: Vec<(NaiveDateTime,OptQuote<Call>)>,
    puts: Vec<(NaiveDateTime,OptQuote<Put>)>,
    unders: Vec<(NaiveDateTime,PriceCalc)>,
}

type UnderType = (NaiveDateTime, PriceCalc);
type CallType = (NaiveDateTime, OptQuote<Call>);
type PutType = (NaiveDateTime, OptQuote<Put>);

const UNDER_SIZE: usize = std::mem::size_of::<UnderType>();
const OPT_SIZE: usize = std::mem::size_of::<CallType>();

pub fn load_all<S:Style>(year:u16, month:u8) -> HistData {
    let (calls_path, puts_path, unders_path) = loader::paths_out(year, month);

    let mut unders_buf = [0_u8; UNDER_SIZE];
    let unders_v = load::<UnderType,UNDER_SIZE>(&unders_path, &mut unders_buf);

    let mut opt_buf = [0_u8; OPT_SIZE];
    let calls_v = load::<CallType,OPT_SIZE>(&calls_path, &mut opt_buf);
    let puts_v = load::<PutType,OPT_SIZE>(&puts_path, &mut opt_buf);
    // let tss_v = Vec::with_capacity(unders_v.len());

    return HistData {
        // tss: tss,
        calls: calls_v,
        puts: puts_v,
        unders: unders_v,
    };
}

fn load<T:Archive,const N: usize>(path: &PathBuf, buf: &mut[u8; N]) -> Vec<T> {
    let mut v: Vec<T> = Vec::new();
    let mut reader = BufReader::new(File::open(path).unwrap());

    loop {
        reader.read_exact(buf);
        // let arch = unsafe { rkyv::archived_value::<T>(&buf[..], 0) };
        // let one: T = arch.deserialize(&mut rkyv::Infallible).unwrap();
        let arch = unsafe { rkyv::archived_root::<T>(buf) };
        let one: T = arch.deserialize(&mut rkyv::Infallible).expect("Could not rkyv deserialize object");
        v.push(one);
        break;
    }

    return v;
}