use crate::types::{Algorithm, FontBitmap};

// Matches a block of pixels with a character
pub fn match_block_with_char(
    block: &[f32; 8 * 16],
    font: &FontBitmap,
    bright_pixels: usize,
    algorithm: &Algorithm,
) -> char {
    match algorithm {
        Algorithm::MaxMult => max_mult(block, font, bright_pixels),
        Algorithm::MinDiff => min_diff(block, font),
        Algorithm::MinDiffSq => min_diff_sq(block, font),
        Algorithm::Gradient => gradient(block, font),
        Algorithm::Correlation => correlation(block, font),
        Algorithm::Ncc => ncc(block, font),
    }
}

fn max_mult(block: &[f32; 128], font: &FontBitmap, bright_pixels: usize) -> char {
    let mut best_match = font.data[0].char;
    let mut best_match_value = f32::MIN;

    // Only take the possible characters if the first char is space
    // If the first char is not space, there can be edge cases
    let chars_to_check = if font.data[0].char == ' ' {
        &font.data[..font.data.partition_point(|l| l.min <= bright_pixels)]
    } else {
        &font.data[..]
    };

    for char in chars_to_check {
        let mut match_value = 0.0;
        for i in 0..128 {
            // Add the value of the pixel to the match value (the block brightness multiplied by the character's brightness in a given position)
            match_value += block[i] * char.data[i];
        }

        // If the match value is greater than the best match value, update the best match
        if match_value > best_match_value {
            best_match_value = match_value;
            best_match = char.char;
        }
    }

    best_match
}

fn min_diff(block: &[f32; 128], font: &FontBitmap) -> char {
    let mut best_match = font.data[0].char;
    let mut less_diffrence = f32::MAX;

    for char in &font.data {
        let mut match_value = 0.0;
        for i in 0..128 {
            match_value += (block[i] - char.data[i]).abs();
        }

        if match_value < less_diffrence {
            less_diffrence = match_value;
            best_match = char.char;
        }
    }

    best_match
}

fn min_diff_sq(block: &[f32; 128], font: &FontBitmap) -> char {
    let mut best_match = font.data[0].char;
    let mut less_diffrence = f32::MAX;

    for char in &font.data {
        let mut match_value = 0.0;
        for i in 0..128 {
            let diff = block[i] - char.data[i];
            match_value += diff * diff;
        }

        if match_value < less_diffrence {
            less_diffrence = match_value;
            best_match = char.char;
        }
    }

    best_match
}

fn gradient(block: &[f32; 128], font: &FontBitmap) -> char {
    let max_char_brightness = font.data[font.data.len() - 1].avg_brightness;
    let min_char_brightness = font.data[0].avg_brightness;

    // Add 0.5 to convert from [-0.5, 0.5] to [0, 1] while allowing to adjust
    // the brightness with arg.midpoint_brightness
    let avg_block_brightness: f32 = block.iter().sum::<f32>() / 128.0 + 0.5;

    let mut best_match = font.data[0].char;
    let mut best_score = f32::MIN;

    for char in &font.data {
        // Normalize char brightnesses to [0, 1] range
        let normalized_char_brightness = (char.avg_brightness - min_char_brightness)
            / (max_char_brightness - min_char_brightness);
        let score = 1.0 - (avg_block_brightness - normalized_char_brightness).abs();

        if score > best_score {
            best_score = score;
            best_match = char.char;
        }
    }

    best_match
}

fn correlation(block: &[f32; 128], font: &FontBitmap) -> char {
    let mut best_match = font.data[0].char;
    let mut best_correlation = f32::MIN;

    // Calculate mean and standard deviation of the block
    let block_mean: f32 = block.iter().sum::<f32>() / 128.0;
    let block_std = (block.iter()
        .map(|&x| (x - block_mean).powi(2))
        .sum::<f32>() / 128.0)
        .sqrt();

    // Skip if block has no variance (all pixels same value)
    if block_std == 0.0 {
        return font.data[0].char;
    }

    for char in &font.data {
        // Skip if character has no variance
        if char.std == 0.0 {
            continue;
        }

        // Calculate Pearson correlation coefficient
        let mut correlation_sum = 0.0;
        for i in 0..128 {
            correlation_sum += (block[i] - block_mean) * (char.data[i] - char.mean);
        }
        let correlation = correlation_sum / (128.0 * block_std * char.std);

        if correlation > best_correlation {
            best_correlation = correlation;
            best_match = char.char;
        }
    }

    best_match
}

fn ncc(block: &[f32; 128], font: &FontBitmap) -> char {
    let mut best_match = font.data[0].char;
    let mut best_ncc = f32::MIN;

    // Calculate the norm (magnitude) of the block
    let block_norm = block.iter()
        .map(|&x| x * x)
        .sum::<f32>()
        .sqrt();

    // Skip if block has zero magnitude
    if block_norm == 0.0 {
        return font.data[0].char;
    }

    for char in &font.data {
        // Skip if character has zero magnitude
        if char.norm == 0.0 {
            continue;
        }

        // Calculate normalized cross-correlation: Σ(block * char) / (||block|| * ||char||)
        let mut dot_product = 0.0;
        for i in 0..128 {
            dot_product += block[i] * char.data[i];
        }
        let ncc_value = dot_product / (block_norm * char.norm);

        if ncc_value > best_ncc {
            best_ncc = ncc_value;
            best_match = char.char;
        }
    }

    best_match
}
