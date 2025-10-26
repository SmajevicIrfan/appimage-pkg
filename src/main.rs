mod cli;
mod config;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let args = Cli::parse();

    let config = match crate::config::load() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    println!("{:?}", config);

    match args.command {
        Commands::Install { source: _, name: _ } => todo!("Implement install"),
        Commands::List { query: _ } => todo!("Implement list"),
        Commands::Remove { name: _ } => todo!("Implement remove"),
    };
}
