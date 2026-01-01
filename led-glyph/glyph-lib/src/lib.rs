use std::error::Error;

struct Glyph {
    width: u8,
    height: u8,
    bitmap: Vec<u16>,
}

impl Glyph {
    pub fn new(size: u8, bitmap: Vec<u16>) -> Result<Glyph, &'static str> {
        // Logic to implement here
        // Glyph would be of size square
        // Validation to run
        // 1. Num of records in bitmap should be equal to size
        if bitmap.len() != size as usize {
            return Err("Not enough row data as per size");
        }
        Ok(Glyph {
            width: size,
            height: size,
            bitmap,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_error_if_row_data_not_equal_to_size_args() {
        todo!()
    }

    #[test]
    fn should_return_glyph_object_if_row_data_equal_to_size_args() {
        todo!()
    }
}
