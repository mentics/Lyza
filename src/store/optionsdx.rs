use log::*;
use regex::Regex;
use serde::Deserialize;
use csv;
use std::path::Path;

use num_traits::Float;
use std::str::FromStr;
use std::fmt::Display;

// 0: [QUOTE_UNIXTIME], [QUOTE_READTIME], [QUOTE_DATE], [QUOTE_TIME_HOURS], [UNDERLYING_LAST], [EXPIRE_DATE], [EXPIRE_UNIX], [DTE],
// 8: [C_DELTA], [C_GAMMA], [C_VEGA], [C_THETA], [C_RHO], [C_IV], [C_VOLUME], [C_LAST], [C_SIZE], [C_BID], [C_ASK],
// 19: [STRIKE],
// 20: [P_BID], [P_ASK], [P_SIZE], [P_LAST], [P_DELTA], [P_GAMMA], [P_VEGA], [P_THETA], [P_RHO], [P_IV], [P_VOLUME],
// 31: [STRIKE_DISTANCE], [STRIKE_DISTANCE_PCT]
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
    pub underlying_last: f32,
    #[serde(rename = "[EXPIRE_DATE]")]
    pub expire_date: String,
    #[serde(rename = "[EXPIRE_UNIX]")]
    pub expire_unix: i64,
    #[serde(rename = "[DTE]")]
    pub dte: String,

    #[serde(rename = "[C_DELTA]", deserialize_with = "blank_to_nan")]
    pub c_delta: f32,
    #[serde(rename = "[C_GAMMA]", deserialize_with = "blank_to_nan")]
    pub c_gamma: f32,
    #[serde(rename = "[C_VEGA]", deserialize_with = "blank_to_nan")]
    pub c_vega: f32,
    #[serde(rename = "[C_THETA]", deserialize_with = "blank_to_nan")]
    pub c_theta: f32,
    #[serde(rename = "[C_RHO]", deserialize_with = "blank_to_nan")]
    pub c_rho: f32,
    #[serde(rename = "[C_IV]", deserialize_with = "blank_to_nan")]
    pub c_iv: f32,
    #[serde(rename = "[C_VOLUME]", deserialize_with = "blank_to_0")]
    pub c_volume: f32,
    #[serde(rename = "[C_LAST]", deserialize_with = "blank_to_nan")]
    pub c_last: f32,
    #[serde(rename = "[C_SIZE]")]
    pub c_size: &'a str,
    #[serde(rename = "[C_BID]")]
    pub c_bid: Option<f32>,
    #[serde(rename = "[C_ASK]")]
    pub c_ask: Option<f32>,

    #[serde(rename = "[STRIKE]")]
    pub strike: f32,

    #[serde(rename = "[P_BID]")]
    pub p_bid: Option<f32>,
    #[serde(rename = "[P_ASK]")]
    pub p_ask: Option<f32>,
    #[serde(rename = "[P_SIZE]")]
    pub p_size: &'a str,
    #[serde(rename = "[P_LAST]", deserialize_with = "blank_to_nan")]
    pub p_last: f32,
    #[serde(rename = "[P_DELTA]", deserialize_with = "blank_to_nan")]
    pub p_delta: f32,
    #[serde(rename = "[P_GAMMA]", deserialize_with = "blank_to_nan")]
    pub p_gamma: f32,
    #[serde(rename = "[P_VEGA]", deserialize_with = "blank_to_nan")]
    pub p_vega: f32,
    #[serde(rename = "[P_THETA]", deserialize_with = "blank_to_nan")]
    pub p_theta: f32,
    #[serde(rename = "[P_RHO]", deserialize_with = "blank_to_nan")]
    pub p_rho: f32,
    #[serde(rename = "[P_IV]", deserialize_with = "blank_to_nan")]
    pub p_iv: f32,
    #[serde(rename = "[P_VOLUME]", deserialize_with = "blank_to_0")]
    pub p_volume: f32,

    #[serde(rename = "[STRIKE_DISTANCE]")]
    pub strike_distance: String,
    #[serde(rename = "[STRIKE_DISTANCE_PCT]")]
    pub strike_distance_pct: String,
}

pub fn load<F,C>(path: &Path, proc: F, ctx:&mut C) where F: Fn(&mut C, &OdxRecord) -> () {
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
    }
    println!("proced: {}, skipped: {}", count, skipped);
}

fn blank_to_nan<'de, D, T>(deserializer: D) -> Result<T, D::Error>
        where D: serde::Deserializer<'de>,
              T: FromStr + Float,
              <T as FromStr>::Err: Display {
    let s = String::deserialize(deserializer)?;
    if s.len() == 0 {
        return Ok(T::nan());
    } else {
        return s.parse::<T>().map_err(serde::de::Error::custom);
    }
}

fn blank_to_0<'de, D, T>(deserializer: D) -> Result<T, D::Error>
        where D: serde::Deserializer<'de>,
              T: FromStr + Float,
              <T as FromStr>::Err: Display {
    let s = String::deserialize(deserializer)?;
    if s.len() == 0 {
        return Ok(T::zero());
    } else {
        return s.parse::<T>().map_err(serde::de::Error::custom);
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
