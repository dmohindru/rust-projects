use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Read, Write};

pub trait DataAccess {
    fn read_data_file(&mut self) -> Result<String, String>;
    fn write_data_file(&mut self, data: Vec<u8>) -> Result<(), String>;
}

pub struct FileDataAccess {
    grid_size: u8,
    out_data_file: String,
}

impl FileDataAccess {
    pub fn new(grid_size: u8, out_data_file: String) -> Self {
        Self {
            grid_size,
            out_data_file,
        }
    }
}

impl DataAccess for FileDataAccess {
    fn read_data_file(&mut self) -> Result<String, String> {
        // TODO come up with a better solution
        let base_path = "/home/dhruv/Programming/rust/rust-projects/led-glyph/glyph-cli/data";
        let file_path = format!(
            "{}/bitmap-{}X{}.json",
            base_path, self.grid_size, self.grid_size
        );
        println!("Reading datfile: {}", &file_path);
        let file = OpenOptions::new()
            .read(true)
            .open(file_path)
            .map_err(|e| e.to_string())?;
        let mut reader = BufReader::new(file);
        let mut input = String::new();
        reader
            .read_to_string(&mut input)
            .map_err(|e| e.to_string())?;
        Ok(input)
    }

    fn write_data_file(&mut self, data: Vec<u8>) -> Result<(), String> {
        println!("Writing to datafile: {}", &self.out_data_file);
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.out_data_file)
            .map_err(|e| e.to_string())?;
        let mut writer = BufWriter::new(file);
        writer.write_all(&data).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
#[derive(Clone)]
pub struct TestDataAccess {
    pub read_result: Result<String, String>,
    pub written_data: Vec<u8>,
    pub write_should_fail: bool,
}

#[cfg(test)]
impl DataAccess for TestDataAccess {
    fn read_data_file(&mut self) -> Result<String, String> {
        self.read_result.clone()
    }

    fn write_data_file(&mut self, data: Vec<u8>) -> Result<(), String> {
        if self.write_should_fail {
            Err(format!("Simulated write error for data: {:?}", data))
        } else {
            self.written_data = data;
            Ok(())
        }
    }
}

#[cfg(test)]
impl TestDataAccess {
    pub fn success(input: &str) -> Self {
        Self {
            read_result: Ok(input.to_string()),
            written_data: Vec::new(),
            write_should_fail: false,
        }
    }

    pub fn read_fail(msg: &str) -> Self {
        Self {
            read_result: Err(msg.to_string()),
            written_data: Vec::new(),
            write_should_fail: false,
        }
    }

    pub fn write_fail(input: &str) -> Self {
        Self {
            read_result: Ok(input.to_string()),
            written_data: Vec::new(),
            write_should_fail: true,
        }
    }
}
