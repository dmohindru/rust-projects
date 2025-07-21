mod cli;

use clap::Parser;
use cli::{TodoCli, Commands};

fn main() {
    let cli = TodoCli::parse();

    match &cli.command {
        Commands::Get { get_command } => {
            println!("Get command params is: {get_command:?}")
        }
    }
}
