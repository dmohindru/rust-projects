use clap::Parser;

use crate::cli::GlyphCli;
mod cli;
fn main() {
    let cli = GlyphCli::parse();
    match cli.char {
        Some(char) => println!(
            "Generating for char {}, grid-size {}, file {}",
            char, cli.grid_size, cli.file
        ),
        None => println!("--char option missing"),
    }

    match cli.string {
        Some(str) => println!(
            "Generating for char {}, grid-size {}, file {}",
            str, cli.grid_size, cli.file
        ),
        None => println!("--string option missing"),
    }

    if cli.info {
        println!(
            "Generating for info {}, grid-size {}, file {}",
            cli.info, cli.grid_size, cli.file
        )
    } else {
        println!("--info option missing")
    }
}
