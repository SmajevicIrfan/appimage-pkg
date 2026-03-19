use std::error::Error;

use crate::AppContext;

pub fn execute(ctx: &AppContext, name: &str) -> Result<(), Box<dyn Error>> {
    println!("Remove {:?}", name);

    Ok(())
}
