mod cli;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Install { source, name } => {
            println!("Install from source(URL: {:?}, File: {:?}), with custom name: {:?}", source.url, source.file, name)
        }
        Commands::List { query } => println!("List with query: {:?}", query),
        Commands::Remove { name } => println!("Remove {:?}", name),
    }
}
