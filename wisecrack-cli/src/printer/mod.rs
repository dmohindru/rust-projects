use crate::cli::OutputFormat;
use crate::data::Data;
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::to_string_pretty;
use std::io::Write;

#[derive(Serialize)]
struct PrinterData {
    message: String,
    time: DateTime<Utc>,
    response_code: u16,
}

pub struct AppPrinter<W: Write> {
    writer: W,
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

    #[cfg(test)]
    pub fn into_writer(self) -> W {
        self.writer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn setup() -> AppPrinter<Cursor<Vec<u8>>> {
        let cursor = Cursor::new(Vec::<u8>::new());
        AppPrinter::new(cursor)
    }

    #[test]
    fn should_print_error_message_in_json_format() {
        let mut app_printer = setup();
        let time_now = Utc::now();
        let test_error_message = String::from("Some Error Message");
        let data = Data {
            data: None,
            time: time_now.clone(),
            error_message: Some(String::from(test_error_message.clone())),
            response_code: 500,
        };
        app_printer.print_data(data.clone(), OutputFormat::Json);
        let printer_data = PrinterData {
            message: test_error_message.clone(),
            time: time_now.clone(),
            response_code: data.response_code,
        };

        // Convert written data back to string
        let output_bytes = app_printer.into_writer().into_inner();
        let output_str = String::from_utf8(output_bytes).unwrap();
        let expected_output_str = format!("{}\n", to_string_pretty(&printer_data).unwrap());
        assert_eq!(expected_output_str, output_str);
    }

    #[test]
    fn should_print_error_message_in_text_format() {
        let mut app_printer = setup();
        let time_now = Utc::now();
        let test_error_message = String::from("Some Error Message");
        let data = Data {
            data: None,
            time: time_now.clone(),
            error_message: Some(String::from(test_error_message.clone())),
            response_code: 500,
        };
        app_printer.print_data(data.clone(), OutputFormat::Text);

        // Convert written data back to string
        let output_bytes = app_printer.into_writer().into_inner();
        let output_str = String::from_utf8(output_bytes).unwrap();
        let expected_output_str = format!("{}\n", data.error_message.unwrap());
        assert_eq!(expected_output_str, output_str);
    }

    #[test]
    fn should_message_in_json_format() {
        let mut app_printer = setup();
        let time_now = Utc::now();
        let message = String::from("Some Data Message");
        let data = Data {
            data: Some(String::from(message.clone())),
            time: time_now.clone(),
            error_message: None,
            response_code: 201,
        };
        app_printer.print_data(data.clone(), OutputFormat::Json);
        let printer_data = PrinterData {
            message: message.clone(),
            time: time_now.clone(),
            response_code: data.response_code,
        };

        // Convert written data back to string
        let output_bytes = app_printer.into_writer().into_inner();
        let output_str = String::from_utf8(output_bytes).unwrap();
        let expected_output_str = format!("{}\n", to_string_pretty(&printer_data).unwrap());
        assert_eq!(expected_output_str, output_str);
    }

    #[test]
    fn should_message_in_text_format() {
        let mut app_printer = setup();
        let time_now = Utc::now();
        let message = String::from("Some Data Message");
        let data = Data {
            data: Some(String::from(message.clone())),
            time: time_now.clone(),
            error_message: None,
            response_code: 201,
        };
        app_printer.print_data(data.clone(), OutputFormat::Text);

        // Convert written data back to string
        let output_bytes = app_printer.into_writer().into_inner();
        let output_str = String::from_utf8(output_bytes).unwrap();
        let expected_output_str = format!("{}\n", data.data.unwrap());
        assert_eq!(expected_output_str, output_str);
    }
}
