use clap::Parser;
use std::io::Stdout;
use std::process;
use wisecrack::cli::WiseCrackCli;
use wisecrack::data::{Data, fetch_dad_jokes, fetch_quote};
use wisecrack::printer::{AppPrinter, PrinterData};

enum AppResult {
    // Exit code 0
    Success,
    //Exit code 1, with Error
    Error(String),
}

impl AppResult {
    pub fn exit(self) -> ! {
        match self {
            AppResult::Success => process::exit(0),
            AppResult::Error(msg) => {
                eprintln!("{}", msg);
                process::exit(1)
            }
        }
    }
}
fn main() {
    let cli = WiseCrackCli::parse();

    // let data = if cli.quote {
    //     fetch_dad_jokes(cli.config)
    // } else {
    //     fetch_quote(cli.config)
    // };
    let mut app_printer = AppPrinter::<Stdout>::new(std::io::stdout());
    let app_result = handle_data(data, &mut app_printer);
    app_result.exit();
}

fn handle_data(data: Data, app_printer: &mut AppPrinter<Stdout>) -> AppResult {
    todo!()
}
