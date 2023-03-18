use log::*;
// use lazy_static::lazy_static;
use regex::Regex;
// use chrono::prelude::{NaiveDateTime};
use serde::Deserialize;
use csv;
use std::path::Path;

// #[path = "../market/types.rs"] mod types;
// use types::*;
use crate::market::types::*;

// [QUOTE_UNIXTIME], [QUOTE_READTIME], [QUOTE_DATE], [QUOTE_TIME_HOURS], [UNDERLYING_LAST], [EXPIRE_DATE], [EXPIRE_UNIX], [DTE],
// [C_DELTA], [C_GAMMA], [C_VEGA], [C_THETA], [C_RHO], [C_IV], [C_VOLUME], [C_LAST], [C_SIZE], [C_BID], [C_ASK],
// [STRIKE],
// [P_BID], [P_ASK], [P_SIZE], [P_LAST], [P_DELTA], [P_GAMMA], [P_VEGA], [P_THETA], [P_RHO], [P_IV], [P_VOLUME],
// [STRIKE_DISTANCE], [STRIKE_DISTANCE_PCT]
#[derive(Debug, Deserialize)]
pub struct OdxRecord<'a> {
    #[serde(rename = "[QUOTE_UNIXTIME]")]
    pub quote_unixtime: i64,
    #[serde(rename = "[QUOTE_READTIME]")]
    pub quote_readtime: String,
    #[serde(rename = "[QUOTE_DATE]")]
    pub quote_date: String,
    #[serde(rename = "[QUOTE_TIME_HOURS]")]
    pub quote_time_hours: String,
    #[serde(rename = "[UNDERLYING_LAST]")]
    pub underlying_last: PriceCalc,
    #[serde(rename = "[EXPIRE_DATE]")]
    pub expire_date: String,
    #[serde(rename = "[EXPIRE_UNIX]")]
    pub expire_unix: i64,
    #[serde(rename = "[DTE]")]
    pub dte: String,

    #[serde(rename = "[C_DELTA]")]
    pub c_delta: f32,
    #[serde(rename = "[C_GAMMA]")]
    pub c_gamma: f32,
    #[serde(rename = "[C_VEGA]")]
    pub c_vega: f32,
    #[serde(rename = "[C_THETA]")]
    pub c_theta: f32,
    #[serde(rename = "[C_RHO]")]
    pub c_rho: f32,
    #[serde(rename = "[C_IV]", deserialize_with = "blank_to_0_f64")]
    pub c_iv: f32,
    #[serde(rename = "[C_VOLUME]", deserialize_with = "blank_to_0_f64")]
    pub c_volume: f32,
    #[serde(rename = "[C_LAST]")]
    pub c_last: PriceCalc,
    #[serde(rename = "[C_SIZE]")]
    pub c_size: &'a str,
    #[serde(rename = "[C_BID]")]
    pub c_bid: PriceCalc,
    #[serde(rename = "[C_ASK]")]
    pub c_ask: PriceCalc,

    #[serde(rename = "[STRIKE]")]
    pub strike: PriceCalc,

    #[serde(rename = "[P_BID]")]
    pub p_bid: PriceCalc,
    #[serde(rename = "[P_ASK]")]
    pub p_ask: PriceCalc,
    #[serde(rename = "[P_SIZE]")]
    pub p_size: &'a str,
    #[serde(rename = "[P_LAST]")]
    pub p_last: PriceCalc,
    #[serde(rename = "[P_DELTA]")]
    pub p_delta: f32,
    #[serde(rename = "[P_GAMMA]")]
    pub p_gamma: f32,
    #[serde(rename = "[P_VEGA]")]
    pub p_vega: f32,
    #[serde(rename = "[P_THETA]")]
    pub p_theta: f32,
    #[serde(rename = "[P_RHO]")]
    pub p_rho: f32,
    #[serde(rename = "[P_IV]", deserialize_with = "blank_to_0_f64")]
    pub p_iv: f32,
    #[serde(rename = "[P_VOLUME]", deserialize_with = "blank_to_0_f64")]
    pub p_volume: f32,

    #[serde(rename = "[STRIKE_DISTANCE]")]
    pub strike_distance: String,
    #[serde(rename = "[STRIKE_DISTANCE_PCT]")]
    pub strike_distance_pct: String,
}

pub fn load<F,C>(path: &Path, proc: F, ctx:C) where F: Fn(C, &OdxRecord) -> () {
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
                        proc(ctx, &rec);
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

lazy_static::lazy_static! {
    static ref RE_SIZE: Regex = Regex::new(r"(\d+)\D+(\d+)").unwrap();
}

pub fn parse_size(s: &str) -> (u32, u32) {
    let captures = RE_SIZE.captures(s).unwrap_or_else(|| panic!("Could not parse size: {s}"));
    let bid = captures[1].parse().unwrap();
    let ask = captures[2].parse().unwrap();
    return (bid, ask);
}
