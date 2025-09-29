use std::io::Write;

use crate::cli::OutputFormat;
pub struct AppPrinter<W: Write> {
    writer: W,
}

pub struct PrinterData {
    message: String,
    response_code: u16,
}

impl PrinterData {
    pub fn new(message: String, response_code: u16) -> Self {
        Self {
            message,
            response_code,
        }
    }
}

impl<W: Write> AppPrinter<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn print_data(&mut self, data: PrinterData, format: OutputFormat) {
        todo!()
    }
}
