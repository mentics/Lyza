pub mod store;
pub mod market;
pub mod backtest;
pub mod general;

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