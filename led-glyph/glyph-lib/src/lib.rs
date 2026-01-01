pub enum AnimationMode {
    Next,
    Scroll,
}

#[derive(PartialEq, Debug)]
pub struct Glyph {
    width: u8,
    height: u8,
    bitmap: Vec<u8>,
}

impl Glyph {
    pub fn new(size: u8, bitmap: Vec<u8>) -> Result<Glyph, &'static str> {
        if bitmap.len() != size as usize {
            return Err("Not enough row data as per size");
        }
        Ok(Glyph {
            width: size,
            height: size,
            bitmap,
        })
    }

    /// Width of the glyph
    pub fn width(&self) -> u8 {
        self.width
    }

    /// Height of the glyph
    pub fn height(&self) -> u8 {
        self.height
    }

    /// Bitmap data as slice
    pub fn bitmap(&self) -> &[u8] {
        &self.bitmap
    }
}

pub fn glyph_animation_frames(glyphs: Vec<Glyph>, mode: AnimationMode) -> Vec<u8> {
    validate_glyph_list(&glyphs);
    todo!()
}

fn validate_glyph_list(glyphs: &Vec<Glyph>) -> bool {
    let first_glyph = &glyphs[0];
    glyphs
        .iter()
        .all(|glyph| glyph.height == first_glyph.height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_error_if_row_data_not_equal_to_size_args() {
        let bitmap: Vec<u8> = vec![0x01, 0x02, 0x03];
        let glyph_result = Glyph::new(4, bitmap);
        assert_eq!(glyph_result, Err("Not enough row data as per size"));
    }

    #[test]
    fn should_return_glyph_object_if_row_data_equal_to_size_args() {
        let bitmap: Vec<u8> = vec![0x01, 0x02, 0x03];
        let glyph = Glyph::new(3, bitmap.clone()).unwrap();
        assert_eq!(glyph.width(), 3);
        assert_eq!(glyph.height(), 3);
        assert_eq!(glyph.bitmap(), &bitmap);
    }
}
