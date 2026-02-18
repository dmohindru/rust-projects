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

fn build_glyph_scroll_animation_frame(glyphs: &Vec<Glyph>) -> Vec<u8> {
    let char_spacing = 1;
    let mut animation_reel: Vec<Vec<bool>> = vec![];
    let first_glyph = glyphs.get(0).unwrap();
    let num_rows = first_glyph.height();
    let width = first_glyph.width() as usize;
    for _ in 0..num_rows {
        animation_reel.push(vec![]);
    }

    for glyph in glyphs.iter() {
        for (index, value) in glyph.bitmap().iter().enumerate() {
            let reel_row = animation_reel.get_mut(index).unwrap();
            reel_row.extend((0..glyph.width()).rev().map(|i| (value & (1 << i)) != 0));
            reel_row.extend((0..char_spacing).map(|_| false));
        }
    }

    // Insert remaining frame bits
    let remaining_bit = first_glyph.width() - char_spacing;
    animation_reel
        .iter_mut()
        .for_each(|v| v.extend((0..remaining_bit).map(|_| false)));

    let max = animation_reel[0].len().saturating_sub(width - 1);
    let mut frames: Vec<u8> = vec![];
    for i in 0..max {
        for row in animation_reel.iter() {
            let slice = row.get(i..i + width).unwrap();
            frames.push(get_bitmap(&slice));
        }
    }

    frames
}

fn get_bitmap(bool_slice: &[bool]) -> u8 {
    let mut result: u8 = 0;
    for bit in bool_slice {
        let bit: u8 = if *bit == true { 1 } else { 0 };
        result = result << 1;
        result = result | bit;
    }

    result
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

    #[test]
    fn should_return_frame_data_for_scroll_animation_mode() {
        let glyph1 = Glyph::new(3, vec![0x04, 0x02, 0x01]).unwrap();
        let glyph2 = Glyph::new(3, vec![0x01, 0x02, 0x04]).unwrap();
        let glyphs = vec![glyph1, glyph2];
        let frame_data = glyph_animation_frames(glyphs, AnimationMode::Scroll);
        assert_eq!(
            vec![
                0x04, 0x02, 0x01, // Frame 1
                0x00, 0x04, 0x02, // Frame 2
                0x00, 0x00, 0x05, // Frame 3
                0x00, 0x01, 0x02, // Frame 4
                0x01, 0x02, 0x04, // Frame 5
                0x02, 0x04, 0x00, // Frame 6
                0x04, 0x00, 0x00, // Frame 7
                0x00, 0x00, 0x00, // Frame 8
            ],
            frame_data
        );
    }
    /*
    A = 100
        010
        001
    B = 001
        010
        100
    animation
    100 0 001 000
    010 0 010 000
    001 0 100 000

    Frames1
    100
    010
    001

    Frame2
    000
    100
    010

    Frame3
    000
    000
    101

    Frame 4
    000
    001
    010

    Frame 5
    001
    010
    100

    Frame 6
    010
    100
    000

    Frame 7
    100
    000
    000

    Frame 8
    000
    000
    000

     */
}
