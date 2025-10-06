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

    let data = if cli.quote {
        fetch_quote()
    } else {
        fetch_dad_jokes()
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
