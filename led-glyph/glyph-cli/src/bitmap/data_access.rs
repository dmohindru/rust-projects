use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

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
        let file_path =
            Self::data_dir().join(format!("bitmap-{}X{}.json", self.grid_size, self.grid_size));
        let file = OpenOptions::new()
            .read(true)
            .open(file_path)
            .map_err(|e| format!("Error reading file with error {}", e.to_string()))?;
        let mut reader = BufReader::new(file);
        let mut input = String::new();
        reader
            .read_to_string(&mut input)
            .map_err(|e| e.to_string())?;
        Ok(input)
    }

    fn write_data_file(&mut self, data: Vec<u8>) -> Result<(), String> {
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

impl FileDataAccess {
    fn data_dir() -> PathBuf {
        dirs::home_dir().unwrap().join(".local/share/glyph")
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
