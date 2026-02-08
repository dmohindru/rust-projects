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
    match mode {
        AnimationMode::Next => build_glyph_next_animation_frames(&glyphs),
        AnimationMode::Scroll => build_glyph_scroll_animation_frame(&glyphs),
    }
}

fn validate_glyph_list(glyphs: &Vec<Glyph>) -> bool {
    let first_glyph = &glyphs[0];
    glyphs
        .iter()
        .all(|glyph| glyph.height == first_glyph.height)
}

fn build_glyph_next_animation_frames(glyphs: &Vec<Glyph>) -> Vec<u8> {
    glyphs.iter().flat_map(|g| g.bitmap.clone()).collect()
}

fn build_glyph_scroll_animation_frame(glyph: &Vec<Glyph>) -> Vec<u8> {
    ///
    /// Logic to implement here
    /// 1. We will have Vector rows of bool (0s and 1s), representing single continuous row of scrolling animation
    /// 2. Number of row will depend on size of glyph e.g for 5X5 it will be five rows
    /// 3. Loop through each glyph and set the content of vector rows as per the bitmap of each glyph row data
    /// 4. At the end of each glyph introduce a blank pixels in each row as a separator between two characters.
    /// 5. If the next glyph is a space then introduce two blank pixels in each row to represent a space between two characters. (To be implemented later)
    /// 6. At the very end of the animation introduce a one full blank glyph as per the size of glyph e.g. 5X5 introduce 5 blank pixels in each row.
    /// 7. Now the whole reel is ready now capture each frame. How to do it
    ///  7.1 Think of it as a picture frame of the size of glyph e.g. 5X5
    ///  7.2 Start at the very start of reel from location 0 (i) to n-1(j). Where n is the size of glyph e.g. 5 or 8 etc and i an j are indexes moving toward left
    ///  7.3 Generate the glyph data by capturing content of reel from i to j for each rows
    ///  7.4 Let to total length of reel be L. Move the capture from to left till j < L
    /// 8. Build the vector to generated frames and return
    ///
    todo!()
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

    #[test]
    fn should_return_frame_data_for_next_animation_mode() {
        let glyph1 = Glyph::new(3, vec![0x01, 0x02, 0x03]).unwrap();
        let glyph2 = Glyph::new(3, vec![0x04, 0x05, 0x06]).unwrap();
        let glyph3 = Glyph::new(3, vec![0x07, 0x08, 0x09]).unwrap();
        let glyphs = vec![glyph1, glyph2, glyph3];
        let frame_data = glyph_animation_frames(glyphs, AnimationMode::Next);
        assert_eq!(
            frame_data,
            vec!(0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09)
        );
    }
}
