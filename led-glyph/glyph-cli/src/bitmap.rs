use crate::cli::CliAnimationMode;
pub trait DataAccess {
    fn read_data_file(&mut self) -> Result<String, String>;
    fn write_data_file(&mut self) -> Result<(), String>;
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
        todo!()
    }

    fn write_data_file(&mut self) -> Result<(), String> {
        todo!()
    }
}

pub struct BitmapRepository<D: DataAccess> {
    data_access: D,
}

impl<D: DataAccess> BitmapRepository<D> {
    pub fn new(data_access: D) -> Self {
        Self { data_access }
    }

    pub fn generate_char_bitmap(grid_size: u8, character: char) -> Vec<u8> {
        todo!()
    }

    pub fn generate_string_bitmap(
        grid_size: u8,
        string: String,
        mode: CliAnimationMode,
    ) -> Vec<u8> {
        todo!()
    }
}

pub fn generate_char_bitmap(grid_size: u8, character: char) -> Vec<u8> {
    todo!()
}

pub fn generate_string_bitmap(grid_size: u8, string: String, mode: CliAnimationMode) -> Vec<u8> {
    todo!()
}
