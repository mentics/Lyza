mod store;
mod market;

fn main() -> Result<(), anyhow::Error> {
    std::fs::create_dir_all("C:/data/log/lyza/")?;
    fern::Dispatch::new()
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file("C:/data/log/lyza/output.log")?).apply()?;

    store::optionsdx::walk();

    return Ok(());
}
