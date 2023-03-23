use lyza;

fn main() -> Result<(), anyhow::Error> {
    crate::lyza::setup_logging()?;

    lyza::store::loader::walk();

    return Ok(());
}
