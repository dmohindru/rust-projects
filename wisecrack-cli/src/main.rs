use clap::Parser;
use std::io::Stdout;
use std::process;
use wisecrack::cli::{OutputFormat, WiseCrackCli};
use wisecrack::data::{Data, fetch_dad_jokes, fetch_quote};
use wisecrack::printer::AppPrinter;

enum AppResult {
    // Exit code 0
    Success,
    //Exit code 1, with Error
    Error,
}

impl AppResult {
    pub fn exit(self) -> ! {
        match self {
            AppResult::Success => process::exit(0),
            AppResult::Error => process::exit(1),
        }
    }
}
fn main() {
    let cli = WiseCrackCli::parse();
    let dad_joke_url = get_api_url("WISECRACK_DAD_JOKE_API", "https://zenquotes.io/api/random");
    let quote_url = get_api_url("WISECRACK_QUOTES_API", "https://icanhazdadjoke.com");

    let data = if cli.quote {
        fetch_quote(&dad_joke_url)
    } else {
        fetch_dad_jokes(&quote_url)
    };
    let mut app_printer = AppPrinter::<Stdout>::new(std::io::stdout());
    let app_result = handle_data(data, &mut app_printer, cli.output);
    app_result.exit();
}

fn handle_data(
    data: Data,
    app_printer: &mut AppPrinter<Stdout>,
    output_format: Option<OutputFormat>,
) -> AppResult {
    let format = output_format.unwrap_or(OutputFormat::Text);
    let error_message = data.error_message.clone();

    app_printer.print_data(data, format);
    match error_message {
        Some(_) => AppResult::Error,
        None => AppResult::Success,
    }
}

fn get_api_url(env_key: &str, default: &str) -> String {
    std::env::var(env_key).unwrap_or_else(|_| default.to_string())
}
