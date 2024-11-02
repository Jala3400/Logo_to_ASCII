use crate::types::FontBitmap;

pub fn match_group_with_letter(group: [[f32; 8]; 16], font: &FontBitmap) -> char {
    let mut best_match = ' ';
    let mut best_match_value = f32::MIN;

    let width = font.width;
    let height = font.height;

    for (letter, letter_pixels) in font.data.iter() {
        let mut match_value = 0.0;
        for y in 0..height {
            for x in 0..width {
                match_value += group[y][x] * letter_pixels.data[y * 8 + x];
            }
        }

        if match_value > best_match_value {
            best_match_value = match_value;
            best_match = *letter;
        }
    }

    best_match
}
