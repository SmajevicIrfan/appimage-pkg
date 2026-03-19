mod cli;
mod commands;
mod config;
mod registry;

use std::error::Error;

use clap::Parser;
use cli::{Cli, Commands};

use crate::{config::Configuration, registry::Registry};

pub struct AppContext {
    pub config: Configuration,
    pub registry: Registry,
}

fn main() {
    let args = Cli::parse();

    let config = match crate::config::load() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let registry = match crate::registry::load() {
        Ok(registry) => registry,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(2);
        }
    };

    let ctx = AppContext { config, registry };

    let _: Result<(), Box<dyn Error>> = match args.command {
        Commands::Install { source, name } => {
            commands::install::execute(&ctx, &source, name.as_deref())
        }
        Commands::List { query } => commands::list::execute(&ctx, query.as_ref()),
        Commands::Remove { name } => commands::remove::execute(&ctx, &name),
    };
}
