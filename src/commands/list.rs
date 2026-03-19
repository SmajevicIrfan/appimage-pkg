use std::error::Error;

use crate::AppContext;

pub fn execute(ctx: &AppContext, query: Option<&String>) -> Result<(), Box<dyn Error>> {
    println!("List with query: {:?}", query);

    Ok(())
}
