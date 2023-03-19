use walkdir::{ WalkDir, DirEntry };
use std::fs::File;
use std::io::{prelude::*, BufWriter};
use std::path::PathBuf;
use chrono::prelude::{NaiveDateTime};
use lazy_static::lazy_static;
use regex::Regex;

// #[path = "optionsdx.rs"] mod optionsdx;
use crate::store::optionsdx;
use crate::market::types::*;

const BASE_DIR:&str = "C:/data/market/optionsdx";

pub fn walk() {
    let iter = WalkDir::new(BASE_DIR).into_iter().filter_map(valid_path);
    for entry in iter {
        let path = entry.path();
        let name = path.file_name().expect("invalid path").to_str().expect("invalid path encoding");
        let (year, month) = parse_ym(name);
        println!("Processing year {year}, month {month}");
        let mut ctx = make_ctx(year, month);
        optionsdx::load(path, proc, &mut ctx);
        //     proc(&mut ctx, &rec);
        // });
        break;
    }
}

pub fn paths_out(year:u16, month:u8) -> (PathBuf, PathBuf, PathBuf, PathBuf) {
    let path_base = PathBuf::from(format!("C:/data/db/lyza/odx-rkyv/{year}{month}/"));
    return (path_base.join("calls.rkyv"), path_base.join("puts.rkyv"), path_base.join("unders.rkyv"), path_base);
}

fn make_ctx(year:u16, month:u8) -> ProcCtx {
    let (calls_path, puts_path, unders_path, path_base) = paths_out(year, month);
    std::fs::create_dir_all(&path_base).expect("Could not create output path {path_base}");
    let calls = File::create(&calls_path).expect("Could not create calls file");
    let puts = File::create(&puts_path).expect("Could not create puts file");
    let unders = File::create(&unders_path).expect("Could not create unders file");
    return ProcCtx {
        calls: BufWriter::new(calls),
        puts: BufWriter::new(puts),
        unders: BufWriter::new(unders),
    };
}

struct ProcCtx {
    calls: BufWriter<File>,
    puts: BufWriter<File>,
    unders: BufWriter<File>,
}

// fn proc(rec: &OdxRecord) {
fn proc(ctx: &mut ProcCtx, rec: &optionsdx::OdxRecord) {
    let ts = NaiveDateTime::from_timestamp_millis (rec.quote_unixtime * 1000).unwrap();
    let under = rec.underlying_last;
    let xpir = NaiveDateTime::from_timestamp_millis (rec.expire_unix * 1000).unwrap().date();
    let strike = rec.strike;

    let (call_size_bid, call_size_ask) = optionsdx::parse_size(rec.c_size);
    let call_opt: Opt<Call> = Opt::new(xpir, strike);
    let call_meta = Meta { delta:rec.c_delta, gamma:rec.c_gamma, vega:rec.c_vega, theta:rec.c_theta, rho:rec.c_rho, iv:rec.c_iv, volume:rec.c_volume };
    let call_quote = Quote { bid:rec.c_bid, ask:rec.c_ask, last:rec.c_last, size_bid:call_size_bid, size_ask:call_size_ask };
    let call_optquote: OptQuote<Call> = OptQuote { opt:call_opt, meta:call_meta, quote:call_quote };
    let call_to_write = (ts, call_optquote);
    let call_bytes = rkyv::to_bytes::<_, 256>(&call_to_write).unwrap();
    ctx.calls.write(&call_bytes).expect("Could not write write");

    let (put_size_bid, put_size_ask) = optionsdx::parse_size(rec.p_size);
    let put_opt: Opt<Call> = Opt::new(xpir, strike);
    let put_meta = Meta { delta:rec.p_delta, gamma:rec.p_gamma, vega:rec.p_vega, theta:rec.p_theta, rho:rec.p_rho, iv:rec.p_iv, volume:rec.p_volume };
    let put_quote = Quote { bid:rec.p_bid, ask:rec.p_ask, last:rec.p_last, size_bid:put_size_bid, size_ask:put_size_ask };
    let put_optquote: OptQuote<Call> = OptQuote { opt:put_opt, meta:put_meta, quote:put_quote };
    let put_to_write = (ts, put_optquote);
    let put_bytes = rkyv::to_bytes::<_, 256>(&put_to_write).unwrap();
    ctx.puts.write(&put_bytes).expect("Could not write put");

    let under_to_write = (ts, under);
    let under_bytes = rkyv::to_bytes::<_, 256>(&under_to_write).unwrap();
    ctx.unders.write(&under_bytes).expect("Could not write under");
}

lazy_static! {
    static ref RE_PATH: Regex = Regex::new(r"spy_15x_(\d{4})(\d{2})\.txt").unwrap();
}

fn valid_path(entry: Result<walkdir::DirEntry, walkdir::Error>) -> Option<DirEntry> {
    return entry.ok().filter(|e| {
        e.file_type().is_file() && e.file_name().to_str().filter(|x| RE_PATH.is_match(x)).is_some()
    });
}

fn parse_ym(s: &str) -> (u16, u8) {
    let caps = RE_PATH.captures(s).unwrap_or_else(|| panic!("Couldn't parse file name {s}"));
    let year = str::parse::<u16>(caps.get(1).expect("invalid year").as_str()).unwrap();
    let month = str::parse::<u8>(caps.get(2).unwrap_or_else(|| panic!("Invalid month {s}")).as_str()).unwrap();
    return (year, month);
}
