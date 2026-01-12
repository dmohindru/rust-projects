mod bitmap;
mod cli;

use crate::{
    bitmap::{BitmapRepository, data_access::FileDataAccess},
    cli::GlyphCli,
};
use clap::Parser;

fn main() {
    if let Err(msg) = run() {
        eprintln!("{msg}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let cli = GlyphCli::parse();

    let file_data_access = FileDataAccess::new(cli.grid_size, cli.file);
    let mut bitmap_repository = BitmapRepository::new(file_data_access);

    match (cli.char, cli.string, cli.info) {
        (Some(ch), _, _) => bitmap_repository.generate_char_bitmap(cli.grid_size, ch),
        (_, Some(s), _) => bitmap_repository.generate_string_bitmap(
            cli.grid_size,
            s,
            cli.mode.expect("--mode is required with --string"),
        ),
        (_, _, true) => {
            todo!()
        }
        _ => Err("No valid action specified".into()),
    }
}
