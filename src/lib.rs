pub mod store;
pub mod market;
pub mod backtest;
pub mod general;
mod macros;

pub fn setup_logging() -> Result<(),anyhow::Error> {
    std::fs::create_dir_all("C:/data/log/lyza/")?;
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        // .append(false)
        .truncate(true)
        .open("C:/data/log/lyza/output.log")?;
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] {} {} {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S%z"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        // fern::log_file("C:/data/log/lyza/output.log")?
        .chain(file)
        .apply()?;
    return Ok(());
}


// use chrono::naive::NaiveDate;
// use std::collections::BTreeMap;
// use rkyv::{Archive, Deserialize, Serialize, with::{AsString, Inline}};

// #[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Eq)]
// // #[archive_attr(derive(Debug, PartialEq))]
// pub struct NewDate(#[with(Inline)] NaiveDate);

// impl Ord for NewDate {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.0.cmp(&other.0)
//     }
// }
// impl PartialOrd for NewDate {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         self.0.partial_cmp(&other.0)
//     }
// }

// #[derive(Archive, Deserialize, Serialize)]
// pub struct TestStruct {
//     chats: BTreeMap<NewDate,u64>,
// }

