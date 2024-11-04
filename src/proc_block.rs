use crate::types::FontBitmap;

pub fn match_group_with_letter(
    group: &[[f32; 8]; 16],
    font: &FontBitmap,
    bright_blocks: usize,
) -> char {
    let mut best_match = ' ';
    let mut best_match_value = f32::MIN;

    let width = font.width;
    let height = font.height;

    for letter in font.data.iter().take_while(|l| l.min < bright_blocks) {
        let mut match_value = 0.0;
        for y in 0..height {
            for x in 0..width {
                match_value += group[y][x] * letter.data[y * 8 + x];
            }
        }

        if match_value > best_match_value {
            best_match_value = match_value;
            best_match = letter.char;
        }
    }

    best_match
}
