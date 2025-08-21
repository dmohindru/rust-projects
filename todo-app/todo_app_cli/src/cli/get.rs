use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum GetCommand {
    All,
    Id(GetIdArgs),
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
