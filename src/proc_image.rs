use crate::{
    args::Args, proc_block::match_group_with_letter, proc_pixel::calc_custom_brightness,
    types::FontBitmap,
};
use enable_ansi_support::enable_ansi_support;
use image::RgbaImage;

// Converts an image to ASCII art
pub fn convert_image(img: &RgbaImage, font: &FontBitmap, args: &Args) -> String {
    // Enable colors
    enable_ansi_support().unwrap();

    // Precalculate needed values
    let height = img.height() as usize;
    let width = img.width() as usize;

    let num_groups_x = (width + 7) / 8;
    let num_groups_y = (height + 15) / 16;

    if args.verbose {
        println!("Image dimensions: {}x{}", width, height);
        println!("Number of characters: {}x{}", num_groups_x, num_groups_y);
    }

    let string_capacity = num_groups_x * num_groups_y * if args.color { 22 } else { 1 };
    let mut result = String::with_capacity(string_capacity);

    let mut group = [0f32; 8 * 16];
    let mut bright_pixels;
    let mut high_pixels;
    let mut full_pixels;
    let mut r: usize;
    let mut g: usize;
    let mut b: usize;

    // Iterate over the groups of pixels
    // Print each character
    for y in 0..num_groups_y {
        for x in 0..num_groups_x {
            bright_pixels = 0;
            high_pixels = 0;
            full_pixels = 0;
            r = 0;
            g = 0;
            b = 0;

            // For each pixel in the group generate the brightness value and store the color
            for by in 0..16 {
                let iy = y * 16 + by;
                for bx in 0..8 {
                    let ix = x * 8 + bx;
                    let cords_block = by * 8 + bx;
                    if iy < height && ix < width {
                        let pixel = img.get_pixel(ix as u32, iy as u32);
                        group[cords_block] = calc_custom_brightness(&pixel, args);
                        if group[cords_block] > -args.midpoint_brightness {
                            bright_pixels += 1;
                            if group[cords_block] >= 0.0 {
                                r += pixel[0] as usize;
                                g += pixel[1] as usize;
                                b += pixel[2] as usize;
                                high_pixels += 1;
                                full_pixels +=
                                    (group[cords_block] == 1.0 - args.midpoint_brightness) as usize;
                            }
                        }
                    } else {
                        // If the pixel is outside of the image, it is considered transparent
                        group[cords_block] = if args.visible {
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
            if args.text_color {
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
            result.push(if full_pixels == 16 * 8 {
                font.data.last().unwrap().char
            } else {
                match_group_with_letter(&group, font, bright_pixels)
            });
        }
        result.push('\n');
    }
    result.push_str("\x1b[0m");

    result
}
