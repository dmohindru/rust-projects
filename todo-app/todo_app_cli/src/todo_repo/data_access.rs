use crate::todo_repo::TodoErrors;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Cursor, Read, Write};
pub trait TodoDataAccess {
    fn read_all(&mut self) -> Result<String, TodoErrors>;
    fn write_all(&mut self, data: String) -> Result<(), TodoErrors>;
}

pub struct FileDataAccess {
    file_path: String,
}

impl FileDataAccess {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
        }
    }
}

impl TodoDataAccess for FileDataAccess {
    fn read_all(&mut self) -> Result<String, TodoErrors> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.file_path)
            .map_err(|e| TodoErrors::TodoGetError(e.to_string()))?;
        let mut reader = BufReader::new(file);
        let mut input = String::new();
        reader
            .read_to_string(&mut input)
            .map_err(|e| TodoErrors::TodoGetError(e.to_string()))?;
        Ok(input)
    }

    fn write_all(&mut self, data: String) -> Result<(), TodoErrors> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.file_path)
            .map_err(|e| TodoErrors::TodoSaveError(e.to_string()))?;
        let mut writer = BufWriter::new(file);
        writer
            .write_all(data.as_bytes())
            .map_err(|e| TodoErrors::TodoSaveError(e.to_string()))
    }
}

#[cfg(test)]
pub struct CursorDataAccess {
    pub reader: Cursor<String>,
    pub writer: Cursor<Vec<u8>>,
}

#[cfg(test)]
impl CursorDataAccess {
    pub fn new(reader: Cursor<String>, writer: Cursor<Vec<u8>>) -> Self {
        Self { reader, writer }
    }
}

#[cfg(test)]
impl TodoDataAccess for CursorDataAccess {
    fn read_all(&mut self) -> Result<String, TodoErrors> {
        let mut input = String::new();
        self.reader
            .read_to_string(&mut input)
            .map_err(|e| TodoErrors::TodoGetError(e.to_string()))?;
        Ok(input)
    }

    fn write_all(&mut self, data: String) -> Result<(), TodoErrors> {
        self.writer
            .write_all(data.as_bytes())
            .map_err(|e| TodoErrors::TodoSaveError(e.to_string()))
    }
}

#[cfg(test)]
pub struct FailingDataAccess {
    pub reader: Cursor<String>,
}

#[cfg(test)]
impl FailingDataAccess {
    pub fn new(reader: Cursor<String>) -> Self {
        Self { reader }
    }
}

#[cfg(test)]
impl TodoDataAccess for FailingDataAccess {
    fn read_all(&mut self) -> Result<String, TodoErrors> {
        let mut input = String::new();
        self.reader
            .read_to_string(&mut input)
            .map_err(|e| TodoErrors::TodoGetError(e.to_string()))?;
        Ok(input)
    }

    fn write_all(&mut self, data: String) -> Result<(), TodoErrors> {
        Err(TodoErrors::TodoSaveError(String::from(format!(
            "Simulated write error for data: {}",
            data
        ))))
    }
}
