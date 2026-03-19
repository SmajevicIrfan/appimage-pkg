use std::error::Error;

use crate::{AppContext, cli::InstallSource};

pub fn execute(
    ctx: &AppContext,
    source: &InstallSource,
    name: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    println!("Install with source: {:?} and name: {:?}", source, name);

    Ok(())
}
