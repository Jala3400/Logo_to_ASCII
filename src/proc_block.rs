use crate::types::{Algorithm, FontBitmap};

// Matches a block of pixels with a character
pub fn match_block_with_letter(
    block: &[f32; 8 * 16],
    font: &FontBitmap,
    bright_blocks: usize,
    algorithm: &Algorithm,
) -> char {
    match algorithm {
        Algorithm::MaxMult => max_mult(block, font, bright_blocks),
        Algorithm::MinDiff => min_diff(block, font),
        Algorithm::MinDiffSq => min_diff_sq(block, font),
    }
}

fn max_mult(block: &[f32; 128], font: &FontBitmap, bright_blocks: usize) -> char {
    let mut best_match = font.data[0].char;
    let mut best_match_value = f32::MIN;

    // Only take the possible characters
    for letter in font.data.iter().take_while(|l| l.min <= bright_blocks) {
        let mut match_value = 0.0;
        for i in 0..128 {
            // Add the value of the pixel to the match value (the block brightness multiplied by the character's brightness in a given position)
            match_value += block[i] * letter.data[i];
        }

        // If the match value is greater than the best match value, update the best match
        if match_value > best_match_value {
            best_match_value = match_value;
            best_match = letter.char;
        }
    }

    best_match
}

fn min_diff(block: &[f32; 128], font: &FontBitmap) -> char {
    let mut best_match = font.data[0].char;
    let mut less_diffrence = f32::MAX;

    for letter in &font.data {
        let mut match_value = 0.0;
        for i in 0..128 {
            match_value += (block[i] - letter.data[i]).abs();
        }

        if match_value < less_diffrence {
            less_diffrence = match_value;
            best_match = letter.char;
        }
    }

    best_match
}

fn min_diff_sq(block: &[f32; 128], font: &FontBitmap) -> char {
    let mut best_match = font.data[0].char;
    let mut less_diffrence = f32::MAX;

    for letter in &font.data {
        let mut match_value = 0.0;
        for i in 0..128 {
            let diff = block[i] - letter.data[i];
            match_value += diff * diff;
        }

        if match_value < less_diffrence {
            less_diffrence = match_value;
            best_match = letter.char;
        }
    }

    best_match
}
