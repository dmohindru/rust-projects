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
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.out_data_file)
            .map_err(|e| e.to_string())?;
        let mut writer = BufWriter::new(file);
        writer.write_all(&data).map_err(|e| e.to_string())
    }
}

// #[cfg(test)]
// use std::io::Cursor;

// #[cfg(test)]
// pub struct CursorDataAccess {
//     pub reader: Cursor<String>,
//     pub writer: Cursor<Vec<u8>>,
// }

// #[cfg(test)]
// impl CursorDataAccess {
//     pub fn new(reader: Cursor<String>, writer: Cursor<Vec<u8>>) -> Self {
//         Self { reader, writer }
//     }
// }

// #[cfg(test)]
// impl DataAccess for CursorDataAccess {
//     fn read_data_file(&mut self) -> Result<String, String> {
//         let mut input = String::new();
//         self.reader
//             .read_to_string(&mut input)
//             .map_err(|e| e.to_string())?;
//         Ok(input)
//     }
//     fn write_data_file(&mut self, data: Vec<u8>) -> Result<(), String> {
//         self.writer.write_all(&data).map_err(|e| e.to_string())
//     }
// }

// #[cfg(test)]
// pub struct FailingDataAccess {
//     pub reader: Cursor<String>,
// }

// #[cfg(test)]
// impl FailingDataAccess {
//     pub fn new(reader: Cursor<String>) -> Self {
//         Self { reader }
//     }
// }

// #[cfg(test)]
// impl DataAccess for FailingDataAccess {
//     fn read_data_file(&mut self) -> Result<String, String> {
//         Err(String::from(format!("Simulated read error")))
//     }
//     fn write_data_file(&mut self, data: Vec<u8>) -> Result<(), String> {
//         Err(String::from(format!(
//             "Simulated write error for data: {:?}",
//             data
//         )))
//     }
// }

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
