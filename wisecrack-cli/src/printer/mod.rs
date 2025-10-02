use std::io::Write;

use crate::cli::OutputFormat;
use crate::data::Data;
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::to_string_pretty;
pub struct AppPrinter<W: Write> {
    writer: W,
}

#[derive(Serialize)]
struct PrinterData {
    message: String,
    time: DateTime<Utc>,
    response_code: u16,
}

impl PrinterData {
    fn new(message: String, time: DateTime<Utc>, response_code: u16) -> Self {
        PrinterData {
            message,
            time,
            response_code,
        }
    }
}

impl<W: Write> AppPrinter<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn print_data(&mut self, data: Data, format: OutputFormat) {
        let printer_data = self.get_printer_data(data);
        let output_str = match format {
            OutputFormat::Text => printer_data.message,
            OutputFormat::Json => to_string_pretty(&printer_data).unwrap(),
        };
        writeln!(self.writer, "{}", output_str).unwrap();
    }

    fn get_printer_data(&mut self, data: Data) -> PrinterData {
        if let Some(error_msg) = data.error_message {
            PrinterData {
                message: error_msg,
                time: data.time,
                response_code: data.response_code,
            }
        } else {
            PrinterData {
                message: data.data.unwrap(),
                time: data.time,
                response_code: data.response_code,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn setup() -> AppPrinter<Cursor<u8>> {
        let cursor = Cursor::new(Vec::<u8>::new());
        // AppPrinter::new(cursor)
    }

    #[test]
    fn should_print_error_message_in_json_format() {}

    #[test]
    fn should_print_error_message_in_text_format() {}

    #[test]
    fn should_message_in_json_format() {}

    #[test]
    fn should_message_in_text_format() {}
}
