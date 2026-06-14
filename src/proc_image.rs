use crate::{
    config::ImageConfig,
    image_ops::apply_negative_to_pixel,
    proc_block::{get_color_for_block, match_block_with_char},
    proc_pixel::calc_custom_brightness,
    types::{CharInfo, FontBitmap, OutputFormat},
};
#[cfg(not(target_arch = "wasm32"))]
use enable_ansi_support::enable_ansi_support;
use image::{Rgba, RgbaImage};
use rayon::prelude::*;
use std::fmt::Write;

// Struct with the necessary information to convert an image to ASCII art
struct ConversionInfo {
    height: usize,
    width: usize,
    num_blocks_x: usize,
    num_blocks_y: usize,
    cell_size: usize,
    vertical_step: usize,
}

fn get_conversion_info(img: &RgbaImage, font: &FontBitmap, config: &ImageConfig) -> ConversionInfo {
    let cell_size = font.cell_size();
    let font_width = font.width;
    let vertical_step = font.vertical_step;

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

    ConversionInfo {
        height,
        width,
        num_blocks_x,
        num_blocks_y,
        cell_size,
        vertical_step,
    }
}

fn get_starting_string(num_blocks_x: usize, num_blocks_y: usize, config: &ImageConfig) -> String {
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

    // Add some extra space for the HTML preamble and closing tags
    let mut result = String::with_capacity(string_capacity + 100);

    // HTML preamble
    if matches!(config.format, OutputFormat::Html) {
        let font_family = match &config.font_name {
            Some(name) => format!("'{}', monospace", name),
            None => "monospace".to_string(),
        };
        let _ = write!(
            result,
            "<pre style=\"font-family:{}; font-size: {}px\">",
            font_family, config.char_size
        );
    }

    result
}

fn close_string(result: &mut String, config: &ImageConfig) {
    match config.format {
        OutputFormat::Ansi => {
            if config.print_color {
                result.push_str("\x1b[0m");
            }
        }
        OutputFormat::Html => {
            result.push_str("</pre>");
        }
    }
}

// Converts an image to ASCII art
pub fn convert_image(img: &RgbaImage, font: &FontBitmap, config: &ImageConfig) -> String {
    // Enable colors (ANSI support is a Windows-only native concern; not needed in WASM)
    #[cfg(not(target_arch = "wasm32"))]
    if config.print_color {
        if let Err(e) = enable_ansi_support() {
            eprintln!("Warning: Could not enable ANSI support: {}", e);
        }
    }

    let conversion_info = get_conversion_info(img, font, config);

    let mut result = get_starting_string(
        conversion_info.num_blocks_x,
        conversion_info.num_blocks_y,
        config,
    );

    process_image_blocks(img, font, &conversion_info, config, &mut result);

    // Closing
    close_string(&mut result, config);

    result
}

#[inline]
fn process_image_blocks(
    img: &RgbaImage,
    font: &FontBitmap,
    conversion_info: &ConversionInfo,
    config: &ImageConfig,
    result: &mut String,
) {
    let outbound_pixel = get_outbound_pixel(config);

    let rows: Vec<String> = (0..conversion_info.num_blocks_y)
        .into_par_iter()
        .map(|y| {
            // Each thread gets its own scratch buffers — no sharing needed
            let mut block = vec![0.0f32; conversion_info.cell_size];
            let mut color_block = config
                .print_color
                .then(|| vec![(0u8, 0u8, 0u8); conversion_info.cell_size]);
            let mut row = String::new();

            for x in 0..conversion_info.num_blocks_x {
                process_block_pixels(
                    img,
                    font,
                    x,
                    y,
                    config,
                    conversion_info,
                    &outbound_pixel,
                    &mut block,
                    &mut color_block,
                    &mut row,
                );
            }
            row.push('\n');
            row
        })
        .collect(); // preserves order — rayon guarantees this

    for row in rows {
        result.push_str(&row);
    }
}

struct OutboundPixel {
    brightness: f32,
    color: (u8, u8, u8),
    bright_pixel: usize,
    full_pixel: usize,
}

fn get_outbound_pixel(config: &ImageConfig) -> OutboundPixel {
    // An out of bounds pixel is considered a transparent pixel
    let bg_color = config.transparent_color;
    let mut bg_pixel = Rgba([bg_color[0], bg_color[1], bg_color[2], 255]);

    if config.negative {
        apply_negative_to_pixel(&mut bg_pixel);
    }

    let brightness = calc_custom_brightness(&bg_pixel, config);
    let bright_pixel = (brightness > -config.midpoint_brightness) as usize;
    let full_pixel = (brightness >= 0.0) as usize;

    OutboundPixel {
        brightness,
        color: (bg_pixel[0], bg_pixel[1], bg_pixel[2]),
        bright_pixel,
        full_pixel,
    }
}

/// Process a single block of pixels, match it with a character and push it to the result
#[inline]
fn process_block_pixels(
    img: &RgbaImage,
    font: &FontBitmap,
    x: usize,
    y: usize,
    config: &ImageConfig,
    conversion_info: &ConversionInfo,
    outbound_pixel: &OutboundPixel,
    block: &mut [f32],
    color_block: &mut Option<Vec<(u8, u8, u8)>>,
    result: &mut String,
) {
    let mut bright_pixels = 0;
    let mut full_pixels = 0;

    // For each pixel in the block generate the brightness value and store the color
    // The block height might be greater than the character height, so iterate by the
    // font_height but calculate the coordinates with the vertical_step.
    for by in 0..font.height {
        let iy = y * conversion_info.vertical_step + by;
        for bx in 0..font.width {
            let ix = x * font.width + bx;
            let cords_block = by * font.width + bx;

            // Handle out-of-bounds pixels (transparent)
            if iy < conversion_info.height && ix < conversion_info.width {
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
                block[cords_block] = outbound_pixel.brightness;

                if let Some(color_block) = color_block {
                    color_block[cords_block] = outbound_pixel.color;
                }

                bright_pixels += outbound_pixel.bright_pixel;
                full_pixels += outbound_pixel.full_pixel;
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
    if config.print_color {
        if let Some(color_block) = color_block {
            let (r, g, b) = get_color_for_block(color_block, block, char_info);
            match config.format {
                OutputFormat::Ansi => {
                    let _ = write!(result, "\x1b[38;2;{};{};{}m{}", r, g, b, char_info.char);
                }

                OutputFormat::Html => {
                    let _ = write!(result, "<span style=\"color:rgb({},{},{})\">", r, g, b);
                    push_escaped_html(char_info.char, result);
                    result.push_str("</span>");
                }
            }
        } else {
            push_character(char_info.char, result, config);
        }
    } else {
        push_character(char_info.char, result, config);
    }
}

#[inline]
fn push_character(c: char, result: &mut String, config: &ImageConfig) {
    if matches!(config.format, OutputFormat::Html) {
        push_escaped_html(c, result);
    } else {
        result.push(c);
    }
}

#[inline]
fn push_escaped_html(c: char, result: &mut String) {
    match c {
        '<' => result.push_str("&lt;"),
        '>' => result.push_str("&gt;"),
        '&' => result.push_str("&amp;"),
        '"' => result.push_str("&quot;"),
        _ => result.push(c),
    }
}
