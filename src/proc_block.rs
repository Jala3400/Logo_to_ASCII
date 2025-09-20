use crate::types::{Algorithm, FontBitmap};

// Matches a block of pixels with a character
pub fn match_group_with_letter(
    group: &[f32; 8 * 16],
    font: &FontBitmap,
    bright_blocks: usize,
    algorithm: &Algorithm,
) -> char {
    match algorithm {
        Algorithm::MaxMult => max_mult(group, font, bright_blocks),
        Algorithm::MinDiff => min_diff(group, font),
        Algorithm::MinDiffSq => min_diff_sq(group, font),
    }
}

fn max_mult(group: &[f32; 8 * 16], font: &FontBitmap, bright_blocks: usize) -> char {
    let mut best_match = font.data[0].char;
    let mut best_match_value = f32::MIN;

    // Only take the possible characters
    for letter in font.data.iter().take_while(|l| l.min < bright_blocks) {
        let mut match_value = 0.0;
        for i in 0..128 {
            // Add the value of the pixel to the match value (the block brightness multiplied by the character's brightness in a given position)
            match_value += group[i] * letter.data[i];
        }

        // If the match value is greater than the best match value, update the best match
        if match_value > best_match_value {
            best_match_value = match_value;
            best_match = letter.char;
        }
    }

    best_match
}

fn min_diff(group: &[f32; 8 * 16], font: &FontBitmap) -> char {
    let mut best_match = font.data[0].char;
    let mut less_diffrence = f32::MAX;

    for letter in &font.data {
        let mut match_value = 0.0;
        for i in 0..128 {
            match_value += ((group[i] + 0.5) - (letter.data[i] + 0.5)).abs();
        }

        if match_value < less_diffrence {
            less_diffrence = match_value;
            best_match = letter.char;
        }
    }

    best_match
}

fn min_diff_sq(group: &[f32; 8 * 16], font: &FontBitmap) -> char {
    let mut best_match = font.data[0].char;
    let mut less_diffrence = f32::MAX;

    for letter in &font.data {
        let mut match_value = 0.0;
        for i in 0..128 {
            let diff = (group[i] + 0.5) - (letter.data[i] + 0.5);
            match_value += diff * diff;
        }

        if match_value < less_diffrence {
            less_diffrence = match_value;
            best_match = letter.char;
        }
    }

    best_match
}
