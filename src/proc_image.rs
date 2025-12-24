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
    let font_width = font.width;
    let font_height = font.height;
    let cell_size = font.cell_size();

    // Precalculate needed values
    let height = img.height() as usize;
    let width = img.width() as usize;

    let num_blocks_x = (width + font_width - 1) / font_width;
    let num_blocks_y = (height + font_height - 1) / font_height;

    if args.verbose {
        println!("Image dimensions: {}x{}", width, height);
        println!("Number of characters: {}x{}", num_blocks_x, num_blocks_y);
    }

    let string_capacity = num_blocks_x * num_blocks_y * if args.print_color { 22 } else { 1 };
    let mut result = String::with_capacity(string_capacity);

    let mut block = vec![0f32; cell_size];
    let mut bright_pixels;
    let mut high_pixels;
    let mut full_pixels;
    let mut r: usize;
    let mut g: usize;
    let mut b: usize;

    // Iterate over the blocks of pixels
    // Print each character
    for y in 0..num_blocks_y {
        for x in 0..num_blocks_x {
            bright_pixels = 0;
            high_pixels = 0;
            full_pixels = 0;
            r = 0;
            g = 0;
            b = 0;

            // For each pixel in the block generate the brightness value and store the color
            for by in 0..font_height {
                let iy = y * font_height + by;
                for bx in 0..font_width {
                    let ix = x * font_width + bx;
                    let cords_block = by * font_width + bx;
                    if iy < height && ix < width {
                        let pixel = img.get_pixel(ix as u32, iy as u32);
                        block[cords_block] = calc_custom_brightness(&pixel, args);
                        if block[cords_block] > -args.midpoint_brightness {
                            bright_pixels += 1;
                            if block[cords_block] >= 0.0 {
                                r += pixel[0] as usize;
                                g += pixel[1] as usize;
                                b += pixel[2] as usize;
                                high_pixels += 1;
                                full_pixels +=
                                    (block[cords_block] == 1.0 - args.midpoint_brightness) as usize;
                            }
                        }
                    } else {
                        // If the pixel is outside of the image, it is considered transparent
                        block[cords_block] = if args.visible {
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

            // If the color flag is set, print the color of the character
            if args.print_color {
                result.push_str(&format!(
                    "{}",
                    if high_pixels > 0 {
                        r /= high_pixels;
                        g /= high_pixels;
                        b /= high_pixels;
                        format!("\x1b[38;2;{};{};{}m", r, g, b)
                    } else {
                        "\x1b[38;2;0;0;0m".to_string()
                    }
                ));
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
