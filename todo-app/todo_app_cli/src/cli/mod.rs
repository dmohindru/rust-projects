pub mod get;

use clap::{Parser, Subcommand};
pub use get::GetCommand;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct TodoCli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get Todos
    Get {
        #[command(subcommand)]
        get_command: GetCommand,
    },
}
