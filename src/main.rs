mod cli;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Install { url, file } => {
            println!("Install from URL: {:?}, File: {:?}", url, file)
        }
        Commands::List { query } => println!("List with query: {:?}", query),
        Commands::Remove { name } => println!("Remove {:?}", name),
    }
}
