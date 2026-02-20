use crate::{
    config::ImageConfig,
    image_ops::apply_negative_to_pixel,
    proc_block::{get_color_for_block, match_block_with_char},
    proc_pixel::calc_custom_brightness,
    types::{CharInfo, FontBitmap, OutputFormat},
};
use enable_ansi_support::enable_ansi_support;
use image::{Rgba, RgbaImage};

// Converts an image to ASCII art
pub fn convert_image(img: &RgbaImage, font: &FontBitmap, config: &ImageConfig) -> String {
    // Enable colors
    if config.print_color {
        if let Err(e) = enable_ansi_support() {
            eprintln!("Warning: Could not enable ANSI support: {}", e);
        }
    }

    // Get font dimensions
    let cell_size = font.cell_size();
    let font_width = font.width;
    let vertical_step = font.vertical_step;

    // Precalculate needed values
    let height = img.height() as usize;
    let width = img.width() as usize;

    let num_blocks_x = (width + font_width - 1) / font_width;
    let num_blocks_y = (height + vertical_step - 1) / vertical_step;

    if config.verbose {
        println!("Image dimensions: {}x{}", width, height);
        println!("Number of characters: {}x{}", num_blocks_x, num_blocks_y);

        let filled_width = num_blocks_x * font_width;
        let filled_height = num_blocks_y * vertical_step;
        let unfilled_width = filled_width - width;
        let unfilled_height = filled_height - height;

        if unfilled_width > 0 || unfilled_height > 0 {
            println!(
                "Unfilled space: {}x{} pixels",
                unfilled_width, unfilled_height
            );
        }
    }

    let color_overhead = match config.format {
        OutputFormat::Ansi => 22,
        OutputFormat::Html => 60,
    };
    let string_capacity = num_blocks_x
        * num_blocks_y
        * if config.print_color {
            color_overhead
        } else {
            1
        };
    let mut result = String::with_capacity(string_capacity);

    // HTML preamble
    if config.print_color && matches!(config.format, OutputFormat::Html) {
        let font_family = match &config.font_name {
            Some(name) => format!("'{}', monospace", name),
            None => "monospace".to_string(),
        };
        result.push_str(&format!("<pre style=\"font-family:{}\">", font_family));
    }

    let mut block = vec![0f32; cell_size];
    let mut color_block = if config.print_color {
        Some(vec![(0u8, 0u8, 0u8); cell_size])
    } else {
        None
    };

    // Iterate over the blocks of pixels and print each character
    for y in 0..num_blocks_y {
        for x in 0..num_blocks_x {
            process_block_pixels(
                img,
                font,
                x,
                y,
                config,
                &mut block,
                &mut color_block,
                &mut result,
            );
        }
        result.push('\n');
    }

    // Closing
    match config.format {
        OutputFormat::Ansi => result.push_str("\x1b[0m"),
        OutputFormat::Html => {
            if config.print_color {
                result.push_str("</pre>");
            }
        }
    }

    result
}

/// Process a single block of pixels, match it with a character and push it to the result
#[inline]
fn process_block_pixels(
    img: &RgbaImage,
    font: &FontBitmap,
    x: usize,
    y: usize,
    config: &ImageConfig,
    block: &mut [f32],
    color_block: &mut Option<Vec<(u8, u8, u8)>>,
    result: &mut String,
) {
    let mut bright_pixels = 0;
    let mut full_pixels = 0;

    let font_width = font.width;
    let font_height = font.height;
    let vertical_step = font.vertical_step;
    let width = img.width() as usize;
    let height = img.height() as usize;

    // For each pixel in the block generate the brightness value and store the color
    // The block height might be greater than the character height, so iterate by the
    // font_height but calculate the coordinates with the vertical_step.
    for by in 0..font_height {
        let iy = y * vertical_step + by;
        for bx in 0..font_width {
            let ix = x * font_width + bx;
            let cords_block = by * font_width + bx;

            // Handle out-of-bounds pixels (transparent)
            if iy < height && ix < width {
                // Process in-bounds pixel
                let pixel = img.get_pixel(ix as u32, iy as u32);
                let brightness = calc_custom_brightness(&pixel, config);
                block[cords_block] = brightness;

                if let Some(color_block) = color_block {
                    color_block[cords_block] = (pixel[0], pixel[1], pixel[2]);
                }

                if brightness > -config.midpoint_brightness {
                    bright_pixels += 1;
                    if brightness >= 0.0 {
                        full_pixels += (brightness == 1.0 - config.midpoint_brightness) as usize;
                    }
                }
            } else {
                // Out-of-bounds (the pixel in the image is transparent)
                // Use background color and apply negative effect if needed
                let bg_color = config.transparent_color;
                let mut bg_pixel = Rgba([bg_color[0], bg_color[1], bg_color[2], 255]);

                if config.negative {
                    apply_negative_to_pixel(&mut bg_pixel);
                }

                let brightness = calc_custom_brightness(&bg_pixel, config);
                block[cords_block] = brightness;

                if let Some(color_block) = color_block {
                    color_block[cords_block] = (bg_pixel[0], bg_pixel[1], bg_pixel[2]);
                }

                if brightness > -config.midpoint_brightness {
                    bright_pixels += 1;
                    if brightness >= 0.0 {
                        full_pixels += (brightness == 1.0 - config.midpoint_brightness) as usize;
                    }
                }
            }
        }
    }

    let char_info =
        match_block_with_char(block, font, bright_pixels, full_pixels, &config.algorithm);

    push_formatted_character(&char_info, result, color_block.as_ref(), block, config);
}

#[inline]
fn push_formatted_character(
    char_info: &CharInfo,
    result: &mut String,
    color_block: Option<&Vec<(u8, u8, u8)>>,
    block: &[f32],
    config: &ImageConfig,
) {
    // If the color flag is set, print the color of the character
    if config.print_color {
        if let Some(color_block) = color_block.as_ref() {
            let (r, g, b) = get_color_for_block(color_block, &block, char_info);
            match config.format {
                OutputFormat::Ansi => {
                    result.push_str(&format!("\x1b[38;2;{};{};{}m", r, g, b));
                    result.push(char_info.char);
                }
                OutputFormat::Html => {
                    let ch = match char_info.char {
                        '<' => "&lt;".to_string(),
                        '>' => "&gt;".to_string(),
                        '&' => "&amp;".to_string(),
                        '"' => "&quot;".to_string(),
                        c => c.to_string(),
                    };
                    result.push_str(&format!(
                        "<span style=\"color:rgb({},{},{})\">{}</span>",
                        r, g, b, ch
                    ));
                }
            }
        } else {
            result.push(char_info.char);
        }
    } else {
        // Append the character
        result.push(char_info.char);
    }
}
