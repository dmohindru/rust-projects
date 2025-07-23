mod cli;
mod todo_repo;
use clap::Parser;
use cli::{Commands, TodoCli};
fn main() {
    let cli = TodoCli::parse();

    match &cli.command {
        Commands::Get { get_command } => {
            println!("Get command params is: {get_command:?}")
        }
        Commands::Add(add_args) => {
            println!("Add command params is: {add_args:?}")
        }
        Commands::Complete(complete_args) => {
            println!("Complete command params is: {complete_args:?}")
        }
        Commands::Delete(delete_args) => {
            println!("Delete command params is: {delete_args:?}")
        }
    }

    if let Some(output) = cli.output {
        println!("Printing output in format: {:?}", output);
    } else {
        println!("Printing output in default format of text");
    }
}
