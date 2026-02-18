mod bitmap;
mod cli;

use crate::{
    bitmap::{BitmapRepository, data_access::FileDataAccess},
    cli::GlyphCli,
};
use clap::Parser;
use std::process;
use std::{fs::OpenOptions, io::Read};

fn main() {
    if let Err(msg) = run() {
        eprintln!("{msg}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let cli = GlyphCli::parse();

    if cli.info {
        print_data_file_info(&cli.file, cli.grid_size)?;
        process::exit(0)
    }

    let file_data_access = FileDataAccess::new(cli.grid_size, cli.file);
    let mut bitmap_repository = BitmapRepository::new(file_data_access);

    match (cli.char, cli.string) {
        (Some(ch), _) => bitmap_repository.generate_char_bitmap(cli.grid_size, ch),
        (_, Some(s)) => bitmap_repository.generate_string_bitmap(
            cli.grid_size,
            s,
            cli.mode.expect("--mode is required with --string"),
        ),
        _ => Err("No valid action specified".into()),
    }
}

fn print_data_file_info(file_path: &String, grid_size: u8) -> Result<(), String> {
    let mut file = OpenOptions::new().read(true).open(file_path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    return match get_frame_count(&buffer, grid_size) {
        Ok(frame_count) => {
            println!("{}", frame_count);
            Ok(())
        }
        Err(message) => {
            eprintln!("{}", message);
            process::exit(1);
        }
    };
}

fn get_frame_count(buffer: &Vec<u8>, grid_size: u8) -> Result<u8, String> {
    if buffer.len() % grid_size as usize != 0 {
        Err(String::from("Invalid frame data"))
    } else {
        let frame_count = buffer.len() as u8 / grid_size;
        Ok(frame_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_error_result_for_invalid_frame_data_and_grid_size() {
        let mut buffer: Vec<u8> = Vec::new();
        for _ in 0..24 {
            buffer.push(0x01);
        }
        let message = get_frame_count(&buffer, 5).unwrap_err();
        assert_eq!(message, String::from("Invalid frame data"));
    }

    #[test]
    fn should_return_ok_result_for_valid_frame_data_and_grid_size() {
        let mut buffer: Vec<u8> = Vec::new();
        for _ in 0..25 {
            buffer.push(0x01);
        }
        let frame_count = get_frame_count(&buffer, 5).unwrap();
        assert_eq!(frame_count, 5);
    }
}
