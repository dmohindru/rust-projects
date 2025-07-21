use clap::{Args, Parser, Subcommand};

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

#[derive(Subcommand, Debug)]
pub enum GetCommand {
    /// Get all Todos
    All,
    /// Get todo by its Id
    Id(GetIdArgs),
    /// Search Todo by its name
    Name(GetNameArgs),
}
#[derive(Args, Debug)]
pub struct GetIdArgs {
    pub todo_id: String,
}

#[derive(Args, Debug)]
pub struct GetNameArgs {
    pub search_string: String,
}