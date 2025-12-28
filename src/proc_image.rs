use crate::{
    args::Args, proc_block::match_block_with_char, proc_pixel::calc_custom_brightness,
    types::FontBitmap,
};
use enable_ansi_support::enable_ansi_support;
use image::RgbaImage;

// Converts an image to ASCII art
pub fn convert_image(img: &RgbaImage, font: &FontBitmap, args: &Args) -> String {
    // Enable colors
    if args.print_color {
        enable_ansi_support().unwrap();
    }

    // Get font dimensions
    let cell_size = font.cell_size();
    let font_width = font.width;
    let font_height = font.height;
    let vertical_step = font.vertical_step;

    // Precalculate needed values
    let height = img.height() as usize;
    let width = img.width() as usize;

    let num_blocks_x = (width + font_width - 1) / font_width;
    let num_blocks_y = (height + vertical_step - 1) / vertical_step;

    if args.verbose {
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

    let string_capacity = num_blocks_x * num_blocks_y * if args.print_color { 22 } else { 1 };
    let mut result = String::with_capacity(string_capacity);
    let mut block = vec![0f32; cell_size];

    // Iterate over the blocks of pixels and print each character
    for y in 0..num_blocks_y {
        for x in 0..num_blocks_x {
            let (bright_pixels, high_pixels, full_pixels, r, g, b) = process_block_pixels(
                img,
                x,
                y,
                font_width,
                font_height,
                vertical_step,
                width,
                height,
                args,
                &mut block,
            );

            // If the color flag is set, print the color of the character
            if args.print_color {
                result.push_str(&format_color_code(r, g, b, high_pixels));
            }

            // Append the character
            result.push(if full_pixels == cell_size {
                font.data.last().unwrap().char
            } else {
                match_block_with_char(&block, font, bright_pixels, &args.algorithm)
            });
        }
        result.push('\n');
    }
    result.push_str("\x1b[0m");

    result
}

/// Process a single block of pixels and return statistics
/// Returns: (bright_pixels, high_pixels, full_pixels, r, g, b)
#[inline]
fn process_block_pixels(
    img: &RgbaImage,
    x: usize,
    y: usize,
    font_width: usize,
    font_height: usize,
    vertical_step: usize,
    width: usize,
    height: usize,
    args: &Args,
    block: &mut [f32],
) -> (usize, usize, usize, usize, usize, usize) {
    let mut bright_pixels = 0;
    let mut high_pixels = 0;
    let mut full_pixels = 0;
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;

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
                let brightness = calc_custom_brightness(&pixel, args);
                block[cords_block] = brightness;

                if brightness > -args.midpoint_brightness {
                    bright_pixels += 1;
                    if brightness >= 0.0 {
                        r += pixel[0] as usize;
                        g += pixel[1] as usize;
                        b += pixel[2] as usize;
                        high_pixels += 1;
                        full_pixels += (brightness == 1.0 - args.midpoint_brightness) as usize;
                    }
                }
            } else {
                // Transparent pixels are only visible when (visible xor negative)
                block[cords_block] = if args.visible != args.negative {
                    r += 255;
                    g += 255;
                    b += 255;
                    high_pixels += 1;
                    full_pixels += 1;
                    1.0 - args.midpoint_brightness
                } else {
                    -args.midpoint_brightness
                }
            }
        }
    }

    (bright_pixels, high_pixels, full_pixels, r, g, b)
}

/// Format ANSI color code from RGB values
#[inline]
fn format_color_code(r: usize, g: usize, b: usize, high_pixels: usize) -> String {
    if high_pixels > 0 {
        format!(
            "\x1b[38;2;{};{};{}m",
            r / high_pixels,
            g / high_pixels,
            b / high_pixels
        )
    } else {
        "\x1b[38;2;0;0;0m".to_string()
    }
}
