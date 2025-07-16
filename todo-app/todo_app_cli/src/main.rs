use clap::Parser;
// use sub commands

// use ArgGroup

#[derive(Parser)]
struct TodoCli {}

fn main() {
    let todo_cli = TodoCli::parse();
}
