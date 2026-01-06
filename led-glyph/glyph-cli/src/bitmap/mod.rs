mod data_access;

use std::collections::HashMap;

use crate::bitmap::data_access::DataAccess;
use crate::cli::CliAnimationMode;
use glyph_lib::{AnimationMode, Glyph};
use serde::{Deserialize, Serialize};
use serde_json::{error::Error, from_str};

// DTOs
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

    pub fn generate_char_bitmap(
        &mut self,
        grid_size: u8,
        character: char,
    ) -> Result<Vec<u8>, String> {
        let input_result = self.data_access.read_data_file();
        let input = match input_result {
            Ok(input) => input,
            Err(message) => return Err(message),
        };

        let bitmap_parse_result: Result<GlyphBitmaps, Error> = from_str(&input);
        let bitmap = match bitmap_parse_result {
            Ok(bitmap) => bitmap,
            Err(e) => return Err(e.to_string()),
        };
        let glyph = match bitmap.glyphs.get(&String::from(character)) {
            Some(glyph) => glyph,
            None => return Err(format!("Bitmap not found for character {}", character)),
        };
        Glyph::new(grid_size, glyph);

        todo!();
    }

    pub fn generate_string_bitmap(
        &mut self,
        grid_size: u8,
        string: String,
        mode: CliAnimationMode,
    ) -> Result<Vec<u8>, String> {
        todo!()
    }
}
