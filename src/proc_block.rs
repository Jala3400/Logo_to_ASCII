use std::collections::HashMap;

pub fn match_group_with_letter(group: [[f32; 8]; 16], font: &HashMap<char, Vec<Vec<f32>>>) -> char {
    let mut best_match = ' ';
    let mut best_match_value = f32::MIN;

    for (letter, letter_pixels) in font.iter() {
        let mut match_value = 0.0;
        for y in 0..16 {
            for x in 0..8 {
                match_value += group[y][x] * letter_pixels[y][x];
            }
        }

        if match_value > best_match_value {
            best_match_value = match_value;
            best_match = *letter;
        }
    }

    best_match
}
