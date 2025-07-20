use clap::Parser;
// use sub commands

// use ArgGroup

/*
1. Split application core logic as a library code
2. Write unit tests
3. Write integration tests
*/

#[derive(Parser)]
struct TodoCli {}

fn main() {
    let todo_cli = TodoCli::parse();
}
