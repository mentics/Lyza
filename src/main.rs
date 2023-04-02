use lyza::{self, store, market::chaintypes::ChainsAll};

fn main() -> Result<(), anyhow::Error> {
    crate::lyza::setup_logging()?;

    // lyza::store::loader::walk();
    let hdall = store::histdata::load_all(None);
    let chall:ChainsAll = store::histdata::make_chall(&hdall);
    store::histdata::save_chall(&chall, "all");

    return Ok(());
}
