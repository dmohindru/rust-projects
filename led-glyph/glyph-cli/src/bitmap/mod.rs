mod data_access;

use std::collections::HashMap;

use crate::bitmap::data_access::DataAccess;
use crate::cli::CliAnimationMode;
use glyph_lib::{AnimationMode, Glyph, glyph_animation_frames};
use serde::{Deserialize, Serialize};
use serde_json::{error::Error, from_str};

#[derive(Serialize, Deserialize)]
struct GlyphMetadata {
    name: String,
    width: u8,
    height: u8,
    bit_order: String,
    version: u8,
}

#[derive(Serialize, Deserialize)]
struct GlyphBitmaps {
    meta: GlyphMetadata,
    glyphs: HashMap<String, Vec<String>>,
}

pub struct BitmapRepository<D: DataAccess> {
    data_access: D,
}

impl<D: DataAccess> BitmapRepository<D> {
    pub fn new(data_access: D) -> Self {
        Self { data_access }
    }

    pub fn generate_char_bitmap(&mut self, grid_size: u8, character: char) -> Result<(), String> {
        let bitmap = self.get_glyph_bitmap()?;
        let glyph_binary_data = Self::get_glyph_bitmap_binary_data(&bitmap, character)?;
        let glyph = Glyph::new(grid_size, glyph_binary_data)?;
        let animation_data = glyph_animation_frames(vec![glyph], AnimationMode::Next);
        self.data_access.write_data_file(animation_data)
    }

    pub fn generate_string_bitmap(
        &mut self,
        grid_size: u8,
        string: String,
        mode: CliAnimationMode,
    ) -> Result<(), String> {
        let bitmap = self.get_glyph_bitmap().unwrap();
        let glyph_binary_data_vec: Vec<Vec<u8>> = string
            .chars()
            .map(|c| Self::get_glyph_bitmap_binary_data(&bitmap, c).unwrap())
            .collect();
        let glyphs: Vec<Glyph> = glyph_binary_data_vec
            .into_iter()
            .map(|g| Glyph::new(grid_size, g).unwrap())
            .collect();
        let animation_mode = match mode {
            CliAnimationMode::Next => AnimationMode::Next,
            CliAnimationMode::Scroll => AnimationMode::Scroll,
        };
        let animation_data = glyph_animation_frames(glyphs, animation_mode);
        self.data_access.write_data_file(animation_data)
    }

    fn get_glyph_bitmap(&mut self) -> Result<GlyphBitmaps, String> {
        let input_result = self.data_access.read_data_file();
        let input = match input_result {
            Ok(input) => input,
            Err(message) => return Err(message),
        };

        let bitmap_parse_result: Result<GlyphBitmaps, Error> = from_str(&input);
        match bitmap_parse_result {
            Ok(bitmap) => Ok(bitmap),
            Err(e) => return Err(e.to_string()),
        }
    }

    fn get_glyph_bitmap_binary_data(
        bitmap: &GlyphBitmaps,
        character: char,
    ) -> Result<Vec<u8>, String> {
        let glyph_vec = match bitmap.glyphs.get(&String::from(character)) {
            Some(glyph) => glyph,
            None => return Err(format!("Bitmap not found for character {}", character)),
        };
        match Self::convert_bitmap_data(&glyph_vec) {
            Ok(binary_data) => Ok(binary_data),
            Err(message) => Err(message),
        }
    }

    fn convert_bitmap_data(string_vec: &Vec<String>) -> Result<Vec<u8>, String> {
        string_vec
            .into_iter()
            .map(|s| Self::binary_string_to_u8(&s))
            .collect()
    }

    fn binary_string_to_u8(bin_string: &String) -> Result<u8, String> {
        if bin_string.len() > 8 {
            return Err("Binary string longer than 8 bits".into());
        }
        u8::from_str_radix(bin_string, 2)
            .map_err(|_| format!("Invalid binary string: {bin_string}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bitmap::data_access::CursorDataAccess;

    #[test]
    fn should_return_u8_for_valid_binary_string() {
        let bin_string = String::from("10101010");
        let bin_data =
            BitmapRepository::<CursorDataAccess>::binary_string_to_u8(&bin_string).unwrap();

        assert_eq!(0b10101010, bin_data);
    }

    #[test]
    fn should_return_error_for_invalid_binary_string() {
        let bin_string = String::from("ab101010");
        let err =
            BitmapRepository::<CursorDataAccess>::binary_string_to_u8(&bin_string).unwrap_err();

        assert_eq!(format!("Invalid binary string: {bin_string}"), err);
    }

    #[test]
    fn should_return_error_for_valid_binary_string_but_size_greater_than_eight() {
        let bin_string = String::from("110101010");
        let err =
            BitmapRepository::<CursorDataAccess>::binary_string_to_u8(&bin_string).unwrap_err();

        assert_eq!("Binary string longer than 8 bits", err);
    }

    #[test]
    fn should_return_u8_vector_for_valid_binary_string_vector() {
        let binary_str_vector = vec![
            String::from("11111111"),
            String::from("00000000"),
            String::from("10101010"),
        ];
        let bin_data_vector =
            BitmapRepository::<CursorDataAccess>::convert_bitmap_data(&binary_str_vector).unwrap();
        let expected_bin_data_vector: Vec<u8> = vec![0b11111111, 0b00000000, 0b10101010];
        assert_eq!(expected_bin_data_vector, bin_data_vector);
    }

    #[test]
    fn should_return_error_for_invalid_binary_string_vector() {
        let binary_str_vector = vec![
            String::from("11111111"),
            String::from("00000000"),
            String::from("ab101010"),
        ];
        let err = BitmapRepository::<CursorDataAccess>::convert_bitmap_data(&binary_str_vector)
            .unwrap_err();
        assert_eq!(format!("Invalid binary string: ab101010"), err);
    }

    #[test]
    fn should_return_error_for_valid_binary_string_vector_but_size_greater_than_eight() {
        let binary_str_vector = vec![
            String::from("11111111"),
            String::from("00000000"),
            String::from("110101010"),
        ];
        let err = BitmapRepository::<CursorDataAccess>::convert_bitmap_data(&binary_str_vector)
            .unwrap_err();
        assert_eq!("Binary string longer than 8 bits", err);
    }

    #[test]
    fn should_return_error_when_reading_glyph_data_file_failed() {}

    #[test]
    fn should_return_error_when_char_bitmap_data_not_found_glyph_data_file() {}

    #[test]
    fn should_return_error_when_char_bitmap_data_not_aligned_with_grid_size() {}

    #[test]
    fn should_return_char_bitmap_data_as_u8_vec_from_glyph_data_file() {}
}
