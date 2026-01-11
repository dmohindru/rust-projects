mod bitmap;
mod cli;
use crate::{
    bitmap::{BitmapRepository, data_access::FileDataAccess},
    cli::GlyphCli,
};
use clap::Parser;

fn main() {
    let cli = GlyphCli::parse();

    let file_data_access = FileDataAccess::new(cli.grid_size, cli.file);
    let mut bitmap_repository = BitmapRepository::new(file_data_access);

    if let Some(ch) = cli.char {
        bitmap_repository
            .generate_char_bitmap(cli.grid_size, ch)
            .unwrap();
    } else if let Some(str) = cli.string {
        bitmap_repository
            .generate_string_bitmap(cli.grid_size, str, cli.mode.unwrap())
            .unwrap();
    } else if cli.info {
        todo!();
    }
}
