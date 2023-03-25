use std::fs::File;
use std::io::{BufReader, Read, ErrorKind};
use std::path::PathBuf;
use crate::general::*;
use crate::market::types::*;
use crate::store::loader;
use rkyv::{self, Deserialize, Archive};

#[derive(Debug)]
pub struct HistData {
    tss: Vec<Timestamp>,
    pub calls: Vec<(Timestamp,OptQuote<Call>)>,
    pub puts: Vec<(Timestamp,OptQuote<Put>)>,
    pub unders: Vec<(Timestamp,PriceCalc)>,
}
// fn make_chall(hd:HistData) -> ChainsAll {
//     let chats = HashMap::new()
//     ChainsAll(chats)
// }

type UnderType = (Timestamp, PriceCalc);
type CallType = (Timestamp, OptQuote<Call>);
type PutType = (Timestamp, OptQuote<Put>);

const UNDER_SIZE: usize = std::mem::size_of::<UnderType>();
const OPT_SIZE: usize = std::mem::size_of::<CallType>();

pub fn load_all(year:u16, month:u8) -> HistData {
    let (calls_path, puts_path, unders_path, _) = loader::paths_out(year, month);

    let mut unders_buf = [0_u8; UNDER_SIZE];
    let unders_v = load::<UnderType,UNDER_SIZE>(&unders_path, &mut unders_buf);

    let mut opt_buf = [0_u8; OPT_SIZE];
    let calls_v = load::<CallType,OPT_SIZE>(&calls_path, &mut opt_buf);
    let puts_v = load::<PutType,OPT_SIZE>(&puts_path, &mut opt_buf);

    return HistData {
        tss: Vec::new(),
        calls: calls_v,
        puts: puts_v,
        unders: unders_v,
    };
}

fn load<T,const N: usize>(path: &PathBuf, buf: &mut[u8; N]) -> Vec<T>
        where T: Archive, rkyv::Archived<T>: Deserialize<T,rkyv::Infallible> {
    let mut v: Vec<T> = Vec::new();
    let mut reader = BufReader::new(File::open(path).unwrap());

    loop {
        match reader.read_exact(buf) {
            Ok(_) => {
                let arch = unsafe { rkyv::archived_root::<T>(buf) };
                let one: T = arch.deserialize(&mut rkyv::Infallible).unwrap();
                v.push(one);
            },
            Err(e) => match e.kind() {
                ErrorKind::UnexpectedEof => break,
                _ => panic!("Error deserializing: {:?}", e)
            }
        };
    }

    return v;
}

// fn load1<T,const N: usize>(buf: &mut[u8; N]) -> T where T: Archive, rkyv::Archived<T>: Deserialize<T,rkyv::Infallible> {
//     let arch = unsafe { rkyv::archived_root::<T>(buf) };
//     let one = arch.deserialize(&mut rkyv::Infallible).unwrap();
//     return one;
// }
