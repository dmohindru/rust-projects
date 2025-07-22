pub mod add;
pub mod complete;
pub mod delete;
pub mod get;

pub use add::AddCommandArgs;
use clap::{Parser, Subcommand, ValueEnum};
pub use complete::CompleteCommandArgs;
pub use delete::DeleteCommandArgs;
pub use get::GetCommand;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct TodoCli {
    #[command(subcommand)]
    pub command: Commands,
    /// Optional output <text|json> defaults to text
    #[arg(short, long)]
    pub output: Option<OutputFormat>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get Todos
    Get {
        #[command(subcommand)]
        get_command: GetCommand,
    },
    /// Add a Todo
    Add(AddCommandArgs),
    /// Complete a Todo
    Complete(CompleteCommandArgs),
    /// Delete a Todo
    Delete(DeleteCommandArgs),
}

#[derive(ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Text,
    Json,
}
