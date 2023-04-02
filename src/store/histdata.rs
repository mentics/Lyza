use regex::Regex;
use speedy::{Writable, Readable, LittleEndian};
use std::fs::File;
use std::io::{BufReader, Read, ErrorKind};
use std::path::PathBuf;
use lazy_static::lazy_static;
use crate::general::*;
use crate::market::types::*;
use crate::store::loader;
use crate::market::chaintypes::ChainsAll;
use crate::store::paths::{PATH_DB, PATH_RKYV};

pub struct HistData {
    pub calls: Vec<(Timestamp,OptQuote<Call>)>,
    pub puts: Vec<(Timestamp,OptQuote<Put>)>,
    pub unders: Vec<(Timestamp,PriceCalc)>,
}

pub fn make_chall(hd:&HistData) -> ChainsAll {
    return ChainsAll::new(&hd.unders, &hd.calls, &hd.puts);
}

pub fn save_chall(chall:&ChainsAll, suffix: &str) {
    let path = path_chall(suffix);
    // let file = File::create(&path)
    //     .unwrap_or_else(|err| panic!("Could not create {path} err: {err}"));
    // chall.write_to_stream(file);
    chall.write_to_file(path).expect("Couldn't write chall");
}

pub fn load_chall(suffix: &str) -> ChainsAll {
    let path = path_chall(suffix);
    let file = File::open(&path)
        .unwrap_or_else(|err| panic!("Could not open chall {path} err: {err}"));
    return ChainsAll::read_from_stream_buffered(file).unwrap();

    // let len = file.metadata().unwrap().len().try_into().unwrap();
    // let mut reader = BufReader::new(file);
    // let mut buf: Vec<u8> = Vec::with_capacity(len);
    // reader.read_to_end(&mut buf)
    //     .unwrap_or_else(|err| panic!("Could not read chall {path} err: {err}"));
    // let arch = unsafe { rkyv::archived_root::<ChainsAll>(&buf) };
    // let chall = arch.deserialize(&mut rkyv::Infallible)
    //     .unwrap_or_else(|err| panic!("Could not deserialize chall {path} err: {err}"));
    // return chall;
}

fn path_chall(suffix:&str) -> String {
    format!("{PATH_DB}chall-{suffix}.rkyv")
}

type UnderType = (Timestamp, PriceCalc);
type CallType = (Timestamp, OptQuote<Call>);
type PutType = (Timestamp, OptQuote<Put>);

const UNDER_SIZE: usize = std::mem::size_of::<UnderType>();
const OPT_SIZE: usize = std::mem::size_of::<CallType>();

lazy_static! {
    static ref RE_RKYV: Regex = Regex::new(r"(\d{4})-(\d{2})").unwrap();
}

fn parse_rkyv_ym(s: &str) -> (u16, u8) {
    let caps = RE_RKYV.captures(s)
        .unwrap_or_else(|| panic!("Couldn't parse file name {s}"));
    let year = str::parse::<u16>(caps.get(1)
        .unwrap_or_else(|| panic!("invalid year {s}")).as_str()).unwrap();
    let month = str::parse::<u8>(caps.get(2)
        .unwrap_or_else(|| panic!("Invalid month {s}")).as_str()).unwrap();
    return (year, month);
}

pub fn load_all() -> HistData {
    println!("Loading all HistData rkyvs");
    let paths = std::fs::read_dir(PATH_RKYV).unwrap();
    let mut yms: Vec<(u16,u8)> = Vec::new();
    for entry in paths {
        let name = entry.unwrap().file_name();
        let s = name.to_str().unwrap();
        if s.len() > 4 {
            yms.push(parse_rkyv_ym(s));
        }
    }
    yms.sort_unstable();

    let mut unders_v: Vec<UnderType> = Vec::new();
    let mut calls_v: Vec<CallType> = Vec::new();
    let mut puts_v: Vec<PutType> = Vec::new();

    // let mut i = 0;
    for (year, month) in yms {
        println!("Loading {year} {month}");
        let (calls_path, puts_path, unders_path, _) = loader::paths_out(year, month);
        load_paths(&mut unders_v, &mut calls_v, &mut puts_v, &calls_path, &puts_path, &unders_path);
        // i += 1;
        // if i >= 4 {
        //     break;
        // }
    }

    return HistData {
        calls: calls_v,
        puts: puts_v,
        unders: unders_v,
    };
}

pub fn load_month(year:u16, month:u8) -> HistData {
    let (calls_path, puts_path, unders_path, _) = loader::paths_out(year, month);

    let mut unders_v: Vec<UnderType> = Vec::new();
    let mut calls_v: Vec<CallType> = Vec::new();
    let mut puts_v: Vec<PutType> = Vec::new();
    load_paths(&mut unders_v, &mut calls_v, &mut puts_v, &calls_path, &puts_path, &unders_path);

    return HistData {
        calls: calls_v,
        puts: puts_v,
        unders: unders_v,
    };
}

// impl<'a,C:Context,T:Readable<'a,C>> Readable<'a,C> for PhantomData<T> {
//     fn read_from< R:Reader<'a,C>>(reader:&mut R) -> Result<Self, C::Error> {
//         let is_call = reader.read_value();
//         if is_call {
//             return Ok(PhantomData::<Call>());
//         } else {
//             return Ok(PhantomData::<Put>());
//         }
//     }

//     #[inline]
//     fn minimum_bytes_needed() -> usize {
//         1
//     }
// }

pub fn load_paths(
            unders_v:&mut Vec<UnderType>, calls_v:&mut Vec<CallType>, puts_v:&mut Vec<PutType>,
            calls_path:&PathBuf, puts_path:&PathBuf, unders_path:&PathBuf) {
    let mut buf = [0u8; UNDER_SIZE];
    load::<UnderType,UNDER_SIZE>(unders_v, &unders_path, &mut buf);
    let mut buf_opt = [0u8; OPT_SIZE];
    load::<CallType,OPT_SIZE>(calls_v, &calls_path, &mut buf_opt);
    load::<PutType,OPT_SIZE>(puts_v, &puts_path, &mut buf_opt);
}

fn load<'a, T:Readable<'a,LittleEndian>+std::fmt::Debug, const N: usize>(
            v:&mut Vec<T>, path: &PathBuf, buf:&mut [u8; N]) {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    // let mut buf = [0u8; UNDER_SIZE];
    loop {
        // let len = T::minimum_bytes_needed();
        match reader.read_exact(buf) {
            Ok(()) => {
                let x = T::read_from_buffer_copying_data(buf).expect("couldn't deserialize");
                v.push(x);
            },
            Err(err) => match err.kind() {
                    ErrorKind::UnexpectedEof => break,
                    _ => panic!("Error deserializing histdata: {:?}", err)
                }
        }
        // // match T::read_from_stream_unbuffered(&file) {
        // match T::read_from_buffer_copying_data(&buf) {
        //     Ok(o) => {
        //         // println!("Pushing to unders: {:?} {:?}", o, file);
        //         v.push(o);
        //     },
        //     Err(err) => {
        //         println!("Reached err {err}");
        //         if err.is_eof() { break } else { panic!("Error deserializing histdata: {:?}", err) }
        //     }
        //     // match err {
        //     //     ErrorKind::UnexpectedEof => break,
        //     //     _ => panic!("Error deserializing histdata: {:?}", err)
        //     // }
        // }
    }
}

// fn load<T,const N: usize>(v:&mut Vec<T>, path: &PathBuf, buf: &mut[u8; N])
//         where T: Archive, rkyv::Archived<T>: Deserialize<T,rkyv::Infallible> {
//     let mut reader = BufReader::new(File::open(path).unwrap());

//     loop {
//         match reader.read_exact(buf) {
//             Ok(_) => {
//                 let arch = unsafe { rkyv::archived_root::<T>(buf) };
//                 let one: T = arch.deserialize(&mut rkyv::Infallible).unwrap();
//                 v.push(one);
//             },
//             Err(e) => match e.kind() {
//                 ErrorKind::UnexpectedEof => break,
//                 _ => panic!("Error deserializing: {:?}", e)
//             }
//         };
//     }
// }
