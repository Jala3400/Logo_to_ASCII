use crate::types::FontBitmap;

// Matches a block of pixels with a character
pub fn match_group_with_letter(
    group: &[f32; 8 * 16],
    font: &FontBitmap,
    bright_blocks: usize,
) -> char {
    let mut best_match = font.data[0].char;
    let mut best_match_value = f32::MIN;

    // Only take the possible characters
    for letter in font.data.iter().take_while(|l| l.min < bright_blocks) {
        let mut match_value = 0.0;
        for y in 0..16 {
            for x in 0..8 {
                let cords = y * 8 + x;
                // Add the value of the pixel to the match value (the block brightness multiplied by the character's brightness in a given position)
                match_value += group[cords] * letter.data[cords];
            }
        }

        // If the match value is greater than the best match value, update the best match
        if match_value > best_match_value {
            best_match_value = match_value;
            best_match = letter.char;
        }
    }

    best_match
}

// // Experimenting with a different approach. It calculates de difference of each pixel.
// pub fn match_group_with_letter_v2(
//     group: &[[f32; 8]; 16],
//     font: &FontBitmap,
//     bright_blocks: usize,
// ) -> char {
//     let mut best_match = font.data[0].char;
//     let mut less_diffrence = f32::MAX;

//     let width = font.width;
//     let height = font.height;

//     for letter in font.data.iter().take_while(|l| l.min < bright_blocks) {
//         let mut match_value = 0.0;
//         for y in 0..height {
//             for x in 0..width {
//                 match_value += ((group[y][x] + 0.5) - (letter.data[y * 8 + x] +0.5)).abs();
//             }
//         }

//         if match_value < less_diffrence {
//             less_diffrence = match_value;
//             best_match = letter.char;
//         }
//     }

//     best_match
// }
