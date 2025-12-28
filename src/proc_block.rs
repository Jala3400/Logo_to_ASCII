use crate::types::{Algorithm, CharInfo, FontBitmap};

// Matches a block of pixels with a character
pub fn match_block_with_char<'a>(
    block: &[f32],
    font: &'a FontBitmap,
    bright_pixels: usize,
    full_pixels: usize,
    algorithm: &Algorithm,
) -> &'a CharInfo {
    match algorithm {
        Algorithm::MaxProd => max_prod(block, font, bright_pixels, full_pixels),
        Algorithm::MinDiff => min_diff(block, font),
        Algorithm::MinDiffSq => min_diff_sq(block, font),
        Algorithm::Gradient => gradient(block, font),
        Algorithm::Correlation => correlation(block, font),
        Algorithm::Ncc => ncc(block, font),
    }
}

/// Maximum Product algorithm for matching a block of pixels with a character
/// Custom algorithm that multiplies the brightness values of the block and the character
fn max_prod<'a>(
    block: &[f32],
    font: &'a FontBitmap,
    bright_pixels: usize,
    full_pixels: usize,
) -> &'a CharInfo {
    // If the block is full, return the fullest character
    if full_pixels == block.len() {
        return font.data.last().unwrap();
    }

    let mut best_match = &font.data[0];
    let mut best_match_value = f32::MIN;
    let cell_size = font.cell_size();

    // Only take the possible characters if the first char is space
    // If the first char is not space, there can be edge cases
    let chars_to_check = if font.data[0].char == ' ' {
        &font.data[..font.data.partition_point(|l| l.min <= bright_pixels)]
    } else {
        &font.data[..]
    };

    for char in chars_to_check {
        let mut match_value = 0.0;
        for i in 0..cell_size {
            // Add the value of the pixel to the match value (the block brightness multiplied by the character's brightness in a given position)
            match_value += block[i] * char.data[i];
        }

        // If the match value is greater than the best match value, update the best match
        if match_value > best_match_value {
            best_match_value = match_value;
            best_match = char;
        }
    }

    best_match
}

/// Minimum Difference algorithm for matching a block of pixels with a character
/// Takes into account the absolute differences between pixel brightness values
fn min_diff<'a>(block: &[f32], font: &'a FontBitmap) -> &'a CharInfo {
    let mut best_match = &font.data[0];
    let mut less_difference = f32::MAX;
    let cell_size = font.cell_size();

    for char in &font.data {
        let mut match_value = 0.0;
        for i in 0..cell_size {
            match_value += (block[i] - char.data[i]).abs();
        }

        if match_value < less_difference {
            less_difference = match_value;
            best_match = char;
        }
    }

    best_match
}

/// Minimum Squared Difference algorithm for matching a block of pixels with a character
/// Takes into account the squared differences between pixel brightness values
fn min_diff_sq<'a>(block: &[f32], font: &'a FontBitmap) -> &'a CharInfo {
    let mut best_match = &font.data[0];
    let mut less_difference = f32::MAX;
    let cell_size = font.cell_size();

    for char in &font.data {
        let mut match_value = 0.0;
        for i in 0..cell_size {
            let diff = block[i] - char.data[i];
            match_value += diff * diff;
        }

        if match_value < less_difference {
            less_difference = match_value;
            best_match = char;
        }
    }

    best_match
}

/// Correlation algorithm for matching a block of pixels with a character
/// Takes into account only the pattern structure, not the brightness level
fn correlation<'a>(block: &[f32], font: &'a FontBitmap) -> &'a CharInfo {
    let mut best_match = &font.data[0];
    let mut best_correlation = f32::MIN;
    let cell_size = font.cell_size();
    let cell_size_f = cell_size as f32;

    // Calculate mean and standard deviation of the block
    let block_mean: f32 = block.iter().sum::<f32>() / cell_size_f;
    let block_std =
        (block.iter().map(|&x| (x - block_mean).powi(2)).sum::<f32>() / cell_size_f).sqrt();

    // Skip if block has no variance (all pixels same value)
    if block_std == 0.0 {
        return &font.data[0];
    }

    for char in &font.data {
        // Skip if character has no variance
        if char.std == 0.0 {
            continue;
        }

        // Calculate Pearson correlation coefficient
        let mut correlation_sum = 0.0;
        for i in 0..cell_size {
            correlation_sum += (block[i] - block_mean) * (char.data[i] - char.mean);
        }
        let correlation = correlation_sum / (cell_size_f * block_std * char.std);

        if correlation > best_correlation {
            best_correlation = correlation;
            best_match = char;
        }
    }

    best_match
}

/// Normalized Cross-Correlation (NCC) algorithm for matching a block of pixels with a character
/// Different from correlation as it takes into account the brightness level, not only the pattern structure
fn ncc<'a>(block: &[f32], font: &'a FontBitmap) -> &'a CharInfo {
    let mut best_match = &font.data[0];
    let mut best_ncc = f32::MIN;
    let cell_size = font.cell_size();

    // Calculate the norm (magnitude) of the block
    let block_norm = block.iter().map(|&x| x * x).sum::<f32>().sqrt();

    // Skip if block has zero magnitude
    if block_norm == 0.0 {
        return &font.data[0];
    }

    for char in &font.data {
        // Skip if character has zero magnitude
        if char.norm == 0.0 {
            continue;
        }

        // Calculate normalized cross-correlation: Σ(block * char) / (||block|| * ||char||)
        let mut dot_product = 0.0;
        for i in 0..cell_size {
            dot_product += block[i] * char.data[i];
        }
        let ncc_value = dot_product / (block_norm * char.norm);

        if ncc_value > best_ncc {
            best_ncc = ncc_value;
            best_match = char;
        }
    }

    best_match
}

/// Gradient algorithm for matching a block of pixels with a character
/// Takes into account only the average brightness of the block
fn gradient<'a>(block: &[f32], font: &'a FontBitmap) -> &'a CharInfo {
    let max_char_brightness = font.data[font.data.len() - 1].avg_brightness;
    let min_char_brightness = font.data[0].avg_brightness;
    let cell_size = font.cell_size() as f32;

    // Add 0.5 to convert from [-0.5, 0.5] to [0, 1] while allowing to adjust
    // If we wanted to do this correctly, we would need to add midpoint_brightness here, but
    // then midpoint_brightness would not have any effect. So by just adding 0.5
    // we let the user adjust the brightness level by changing midpoint_brightness
    let avg_block_brightness: f32 = block.iter().sum::<f32>() / cell_size + 0.5;

    let mut best_match = &font.data[0];
    let mut best_score = f32::MIN;

    for char in &font.data {
        // Normalize char brightnesses to [0, 1] range
        let normalized_char_brightness = (char.avg_brightness - min_char_brightness)
            / (max_char_brightness - min_char_brightness);
        let score = 1.0 - (avg_block_brightness - normalized_char_brightness).abs();

        if score > best_score {
            best_score = score;
            best_match = char;
        }
    }

    best_match
}

/// Gets the color that best matches a block of pixels given a character
pub fn get_color_for_block(
    block: &[(u8, u8, u8)],
    block_bitmap: &[f32],
    char_info: &CharInfo,
) -> (u8, u8, u8) {
    // For each pixel in the character, get the color of the block multiplied by the brightness value
    let mut r = 0usize;
    let mut g = 0usize;
    let mut b = 0usize;
    let mut count = 0usize;
    let char_bitmap = &char_info.data;

    for i in 0..char_bitmap.len() {
        // Only record color if both the block and the character have brightness in that pixel
        // The character must be bright to only consider visible pixels
        // The block must be bright to avoid considering dark pixels
        if char_bitmap[i] > 0.0 && block_bitmap[i] > 0.0 {
            let (br, bg, bb) = block[i];
            r += br as usize;
            g += bg as usize;
            b += bb as usize;
            count += 1;
        }
    }

    if count == 0 {
        (0, 0, 0)
    } else {
        ((r / count) as u8, (g / count) as u8, (b / count) as u8)
    }
}
