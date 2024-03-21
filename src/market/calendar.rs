use lazy_static::lazy_static;
use std::{path::PathBuf, ops::RangeInclusive, sync::Mutex, collections::BTreeMap, fs::File, io::BufReader};
use chrono::{NaiveTime, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::market::data::dir_data;

pub fn market_close(date: NaiveDate) -> NaiveTime {
    let mkt = &*MKT_TIME.lock().unwrap();
    let mt = mkt.get(&date).unwrap();
    return *mt.opens.end();
}

//////////////////////////

lazy_static! {
    static ref CAL_DIR: PathBuf = dir_data(&["cal"]).to_path_buf();
    static ref TIME_PATH: PathBuf = CAL_DIR.join("marktime.json");
    static ref DUR_PATH: PathBuf = CAL_DIR.join("markdur.json");

    static ref MKT_TIME: Mutex<BTreeMap<NaiveDate,MarketTime>> = {
        Mutex::new(load_mkttime())
    };
}

type InterTime = RangeInclusive<NaiveTime>;

#[derive(Serialize,Deserialize)]
enum MTStatus {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "weekend")]
    Weekend,
    #[serde(rename = "holiday")]
    Holiday
}

#[derive(Serialize,Deserialize)]
#[serde(from = "OldMarketTime")]
struct MarketTime {
    status:MTStatus,
    pres:InterTime,
    opens:InterTime,
    posts:InterTime,
}

#[derive(Deserialize)]
struct OldInterTime {
    first: NaiveTime,
    last: NaiveTime,
}

#[derive(Deserialize)]
struct OldMarketTime {
    status:MTStatus,
    // #[serde(deserialize_with = "it_deser")]
    pres:OldInterTime,
    opens:OldInterTime,
    posts:OldInterTime,
}
impl From<OldMarketTime> for MarketTime {
    fn from(x: OldMarketTime) -> Self {
        MarketTime {
            status: x.status,
            pres: InterTime::from(x.pres),
            opens: InterTime::from(x.opens),
            posts: InterTime::from(x.posts),
        }
    }
}
impl From<OldInterTime> for InterTime {
    fn from(x: OldInterTime) -> Self {
        InterTime::new(x.first, x.last)
    }
}

fn load_mkttime() -> BTreeMap<NaiveDate,MarketTime> {
    print!("Loading calendar...");
    let reader = BufReader::new(File::open(TIME_PATH.as_path()).unwrap());
    let v:BTreeMap<NaiveDate,MarketTime> = serde_json::from_reader(reader).unwrap();
    println!("done.");
    return v;
}
