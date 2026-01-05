mod bitmap;
mod cli;
use crate::{
    bitmap::{generate_char_bitmap, generate_string_bitmap},
    cli::GlyphCli,
};
use clap::Parser;

fn main() {
    let cli = GlyphCli::parse();
    /* Steps involved
    1. Read data file for bitmaps
    2. Create GlyphCli from glyph-lib
    3. Generate a bitmap vector
    4. Write to the data file
    */

    if let Some(ch) = cli.char {
        let bitmap = generate_char_bitmap(cli.grid_size, ch);
        write_data_file(&cli.file, bitmap);
    }

    if let Some(str) = cli.string {
        let bitmap = generate_string_bitmap(cli.grid_size, str, cli.mode.unwrap());
        write_data_file(&cli.file, bitmap);
    }

    if cli.info {
        verify_data_file(&cli.file, cli.grid_size);
    }
}

fn write_data_file(file_name: &String, data: Vec<u8>) {
    todo!()
}

fn verify_data_file(file_name: &String, grid_size: u8) {
    todo!()
}
