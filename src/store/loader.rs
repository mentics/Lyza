use log::*;
use walkdir::{ WalkDir, DirEntry };
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use lazy_static::lazy_static;
use regex::Regex;
use speedy::Writable;

use crate::general::*;
use crate::store::optionsdx;
use crate::market::types::*;
use crate::store::paths::*;

pub fn walk(filter:Option<fn((u16,u8))->bool>) {
    info!("Walking paths");
    let iter = WalkDir::new(PATH_ODX).into_iter().filter_map(valid_path);
    for entry in iter {
        let path = entry.path();
        let name = path.file_name().expect("invalid path").to_str().expect("invalid path encoding");
        let ym = parse_ym(name);
        let (year, month) = ym;
        if filter.map_or(true, |f| f(ym)) {
            println!("Processing year {year}, month {month}");
            info!("Processing year {year}, month {month}");
            let mut ctx = make_ctx(year, month);
            optionsdx::load(path, proc, &mut ctx);
        }
    }
}

pub fn paths_out(year:u16, month:u8) -> (PathBuf, PathBuf, PathBuf, PathBuf) {
    let path_base = PathBuf::from(format!("{PATH_RKYV}{:04}-{:02}/", year, month));
    return (path_base.join("calls.rkyv"), path_base.join("puts.rkyv"), path_base.join("unders.rkyv"), path_base);
}

fn make_ctx(year:u16, month:u8) -> ProcCtx {
    let (calls_path, puts_path, unders_path, path_base) = paths_out(year, month);
    std::fs::create_dir_all(&path_base).expect("Could not create output path {path_base}");
    let calls = File::create(&calls_path).expect("Could not create calls file");
    let puts = File::create(&puts_path).expect("Could not create puts file");
    let unders = File::create(&unders_path).expect("Could not create unders file");
    return ProcCtx {
        ts_prev: TS_ZERO,
        calls: BufWriter::new(calls),
        puts: BufWriter::new(puts),
        unders: BufWriter::new(unders),
    };
}

struct ProcCtx {
    ts_prev: Timestamp,
    calls: BufWriter<File>,
    puts: BufWriter<File>,
    unders: BufWriter<File>,
}

// fn proc(rec: &OdxRecord) {
fn proc(ctx: &mut ProcCtx, rec: &optionsdx::OdxRecord) {
    let ts = Timestamp (rec.quote_unixtime * 1000);
    let under = rec.underlying_last;
    let xpir = ExpirDate::from_naive(chrono::naive::NaiveDateTime::from_timestamp_millis(rec.expire_unix * 1000).unwrap().date());
    // let strike = PriceCalc(rec.strike);
    let strike = rec.strike;

    // TODO: greeks should probably use Missing because they can be NaN

    if let (Some(c_bid), Some(c_ask)) = (rec.c_bid, rec.c_ask) {
        let (call_size_bid, call_size_ask) = optionsdx::parse_size(rec.c_size);
        let call_opt: Opt<Call> = Opt::new(xpir, to_strike(strike));
        let call_meta = Meta { delta:rec.c_delta, gamma:rec.c_gamma, vega:rec.c_vega, theta:rec.c_theta, rho:rec.c_rho, iv:rec.c_iv, volume:rec.c_volume };
        let call_quote = QuoteOption {
            quote:Quote {
                bisk:BidAsk { bid:c_bid, ask:c_ask },
                last:rec.c_last, size_bid:call_size_bid, size_ask:call_size_ask
            },
            meta:call_meta
        };
        let call_optquote: OptQuote<Call> = OptQuote { opt:call_opt, quote:call_quote };
        let call_to_write = (ts, call_optquote);
        call_to_write.write_to_stream(&mut ctx.calls).unwrap();
    }

    if let (Some(p_bid), Some(p_ask)) = (rec.p_bid, rec.p_ask) {
        let (put_size_bid, put_size_ask) = optionsdx::parse_size(rec.p_size);
        let put_opt: Opt<Call> = Opt::new(xpir, to_strike(strike));
        let put_meta = Meta { delta:rec.p_delta, gamma:rec.p_gamma, vega:rec.p_vega, theta:rec.p_theta, rho:rec.p_rho, iv:rec.p_iv, volume:rec.p_volume };
        let put_quote = QuoteOption {
            quote:Quote {
                bisk:BidAsk { bid:p_bid, ask:p_ask },
                last:rec.p_last, size_bid:put_size_bid, size_ask:put_size_ask },
            meta:put_meta
        };
        let put_optquote: OptQuote<Call> = OptQuote { opt:put_opt, quote:put_quote };
        let put_to_write = (ts, put_optquote);
        put_to_write.write_to_stream(&mut ctx.puts).unwrap();
    }

    if ts != ctx.ts_prev {
        let under_to_write = (ts, under);
        under_to_write.write_to_stream(&mut ctx.unders).unwrap();
        // println!("{}{}", ts, ctx.ts_prev);
        ctx.ts_prev = ts;
    }
}

use std::io::Cursor;
use speedy::*;
pub fn test_write() {
    let val:(Timestamp,PriceCalc) = (Timestamp(17), 13.3);
    let mut buf:[u8; 12] = [0; 12];
    val.write_to_buffer(&mut buf).unwrap();
    println!("{:?}", buf);
    let x1 = super::histdata::UnderType::read_from_buffer_copying_data(&buf).unwrap();
    println!("{:?}", x1);
    let mut c:Cursor<Vec<u8>> = Cursor::new(Vec::with_capacity(16));
    x1.write_to_stream(&mut c).unwrap();
    println!("{:?}", c);
    c.set_position(0);
    let x2 = super::histdata::UnderType::read_from_stream_buffered(&mut c).unwrap();
    println!("{:?}", x2);
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
