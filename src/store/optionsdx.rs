use log::*;
use walkdir::{ WalkDir, DirEntry };
use lazy_static::lazy_static;
use regex::Regex;
use std::path::Path;
use chrono::prelude::*;
use serde::Deserialize;
use csv;

const BASE_DIR:&str = "C:/data/market/optionsdx";

pub fn walk() {
    let iter = WalkDir::new(BASE_DIR).into_iter().filter_map(valid_path);
    for entry in iter {
        load(entry.path())
        // println!("{}", entry.path().display());
    }
}

fn valid_path(entry: Result<walkdir::DirEntry, walkdir::Error>) -> Option<DirEntry> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"spy_15x_([^/\\]+)\.txt").unwrap();
    }

    return entry.ok().filter(|e| {
        e.file_type().is_file() && e.file_name().to_str().filter(|x| RE.is_match(x)).is_some()
    });
}

// [QUOTE_UNIXTIME], [QUOTE_READTIME], [QUOTE_DATE], [QUOTE_TIME_HOURS], [UNDERLYING_LAST], [EXPIRE_DATE], [EXPIRE_UNIX], [DTE],
// [C_DELTA], [C_GAMMA], [C_VEGA], [C_THETA], [C_RHO], [C_IV], [C_VOLUME], [C_LAST], [C_SIZE], [C_BID], [C_ASK],
// [STRIKE],
// [P_BID], [P_ASK], [P_SIZE], [P_LAST], [P_DELTA], [P_GAMMA], [P_VEGA], [P_THETA], [P_RHO], [P_IV], [P_VOLUME],
// [STRIKE_DISTANCE], [STRIKE_DISTANCE_PCT]
#[derive(Debug, Deserialize)]
struct OdxRecord {
    #[serde(rename = "[QUOTE_UNIXTIME]")]
    quote_unixtime: u64,
    #[serde(rename = "[QUOTE_READTIME]")]
    quote_readtime: String,
    #[serde(rename = "[QUOTE_DATE]")]
    quote_date: String,
    #[serde(rename = "[QUOTE_TIME_HOURS]")]
    quote_time_hours: String,
    #[serde(rename = "[UNDERLYING_LAST]")]
    underlying_last: f32,
    #[serde(rename = "[EXPIRE_DATE]")]
    expire_date: NaiveDate,
    #[serde(rename = "[EXPIRE_UNIX]")]
    expire_unix: String,
    #[serde(rename = "[DTE]")]
    dte: String,

    #[serde(rename = "[C_GAMMA]")]
    c_gamma: f32,
    #[serde(rename = "[C_VEGA]")]
    c_vega: f32,
    #[serde(rename = "[C_THETA]")]
    c_theta: f32,
    #[serde(rename = "[C_RHO]")]
    c_rho: f32,
    #[serde(rename = "[C_IV]", deserialize_with = "blank_to_0_f64")]
    c_iv: f32,
    #[serde(rename = "[C_VOLUME]", deserialize_with = "blank_to_0_f64")]
    c_volume: f32,
    #[serde(rename = "[C_LAST]")]
    c_last: f32,
    #[serde(rename = "[C_SIZE]")]
    c_size: String,
    #[serde(rename = "[C_BID]")]
    c_bid: f32,
    #[serde(rename = "[C_ASK]")]
    c_ask: f32,

    #[serde(rename = "[STRIKE]")]
    strike: f32,

    #[serde(rename = "[P_BID]")]
    p_bid: f32,
    #[serde(rename = "[P_ASK]")]
    p_ask: f32,
    #[serde(rename = "[P_SIZE]")]
    p_size: String,
    #[serde(rename = "[P_LAST]")]
    p_last: f32,
    #[serde(rename = "[P_DELTA]")]
    p_delta: f32,
    #[serde(rename = "[P_GAMMA]")]
    p_gamma: f32,
    #[serde(rename = "[P_VEGA]")]
    p_vega: f32,
    #[serde(rename = "[P_THETA]")]
    p_theta: f32,
    #[serde(rename = "[P_RHO]")]
    p_rho: f32,
    #[serde(rename = "[P_IV]", deserialize_with = "blank_to_0_f64")]
    p_iv: f32,
    #[serde(rename = "[P_VOLUME]", deserialize_with = "blank_to_0_f64")]
    p_volume: f32,

    #[serde(rename = "[STRIKE_DISTANCE]")]
    strike_distance: String,
    #[serde(rename = "[STRIKE_DISTANCE_PCT]")]
    strike_distance_pct: String,
}

fn parse_ym(s: &str) -> (i32, i32) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"spy_15x_(\d{4})(\d{2})\.txt").unwrap();
    }
    let caps = RE.captures(s).unwrap_or_else(|| panic!("Couldn't parse file name {s}"));
    let year = str::parse::<i32>(caps.get(1).expect("invalid year").as_str()).unwrap();
    let month = str::parse::<i32>(caps.get(2).unwrap_or_else(|| panic!("Invalid month {s}")).as_str()).unwrap();
    return (year, month);
}

fn make_ctx(year:i32, month:i32) -> ProcCtx {
    let base = format!("C:/data/db/lyza/odx-rkyv/{year}{month}/");
    let path_base = std::path::Path::new(&base);
    std::fs::create_dir_all(path_base).expect("Could not create output path {path_base}");
    let mut calls = File::create(path_base.join("calls.rkyv")).expect("Could not create calls file");
    let mut puts = File::create(path_base.join("puts.rkyv")).expect("Could not create puts file");
    let mut unders = File::create(path_base.join("unders.rkyv")).expect("Could not create unders file");
    return ProcCtx {
        calls: BufWriter::new(calls),
        puts: BufWriter::new(puts),
        unders: BufWriter::new(unders),
    };
}

fn load(path: &Path) {
    let name = path.file_name().expect("invalid path").to_str().expect("invalid path encoding");
    let (year, month) = parse_ym(name);

    println!("Processing year {year}, month {month}");
    let mut ctx = make_ctx(year, month);

    // let mut reader = csv::Reader::from_path(path).expect("Could not open csv");
    let mut reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(path).expect("Couldn't create csv reader");

    let mut raw_record = csv::ByteRecord::new();
    let headers = reader.byte_headers().expect("Couldn't read headers").clone();

    let mut count = 0;
    let mut skipped = 0;
    loop {
        match reader.read_byte_record(&mut raw_record) {
        // match raw_record.deserialize(Some(&headers)) {
            Ok(b) => {
                if !b { break };
                match raw_record.deserialize(Some(&headers)) {
                    Ok(rec) => {
                        count += 1;
                        proc(&mut ctx, &rec)
                    },
                    Err(err) => {
                        skipped += 1;
                        error!("Error deserializing: {:?}\n{:?}", err, raw_record);
                    },
                }
            },
            Err(err) => {
                skipped += 1;
                error!("Error reading row: {:?}\n{:?}", err, raw_record);
            },
        }
        break;
    }
    println!("proced: {}, skipped: {}", count, skipped);
}

fn blank_to_0_f64<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<f32, D::Error> {
    let s = String::deserialize(deserializer)?;
    if s.len() == 0 {
        return Ok(0.0);
    } else {
        return s.parse::<f32>().map_err(serde::de::Error::custom);
    }
}

use rkyv::{Archive, Serialize};
#[path = "../market/types.rs"] mod types;
use types::*; // {PriceCalc, Opt, Call};
// use std::mem::size_of;

use std::fs::File;
use std::io::{prelude::*, BufWriter};

struct ProcCtx {
    calls: BufWriter<File>,
    puts: BufWriter<File>,
    unders: BufWriter<File>,
}

// fn proc(rec: &OdxRecord) {
fn proc(ctx: &mut ProcCtx, rec: &OdxRecord) {
    // info!("PROC: {:?}", rec)
    let opt1: Opt<Call> = Opt::new(NaiveDate::from_ymd_opt(2023,3,16).expect("invalid naive date literal"), PriceCalc(1.23));

    // println!("size of NaiveDate {}, size of PriceCalc {}, size of f64 {}, size of Opt<Call> {}",
    //     size_of::<NaiveDate>(), size_of::<PriceCalc>(), size_of::<f32>(), size_of::<Opt::<Call>>()
    // );

    let bytes = rkyv::to_bytes::<_, 256>(&opt1).unwrap();
    ctx.calls.write(&bytes);
}
