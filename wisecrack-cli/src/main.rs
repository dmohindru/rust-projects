use clap::Parser;
use wisecrack::cli::{OutputFormat, WiseCrackCli};
fn main() {
    let cli = WiseCrackCli::parse();

    let config_path = match cli.config {
        Some(config_file_path) => config_file_path,
        None => String::from("Using default file path"),
    };
    let output_format = match cli.output {
        Some(output_format) => output_format,
        None => OutputFormat::Text,
    };
    let format = match output_format {
        OutputFormat::Json => "json",
        OutputFormat::Text => "text",
    };
    println!("Quote: {}", cli.quote);
    println!("Joke: {}", cli.joke);
    println!("config_path: {}", config_path);
    println!("output_format: {}", format);
}
