use crate::{
    args::Args,
    proc_pixel::{brightness_difference, calculate_brightness, hue_difference},
    types::{BorderCriteria, FontBitmap},
};
use image::{EncodableLayout, RgbaImage};

// Detects the borders of an image and paints them black
pub fn borders_image(img: &mut RgbaImage, args: &Args, thickness: u32) {
    // Get the borders (difference color or brightness)
    let borders = detect_borders(&img, args);

    // Paint the borders
    paint_borders(img, borders, thickness);
}

// Detects the borders of an image
// Unified function that handles all detection modes in a single if statement
fn detect_borders(img: &image::RgbaImage, args: &Args) -> Vec<(u32, u32)> {
    // Return empty if no border criteria is set
    let Some(criteria) = &args.border_criteria else {
        return Vec::new();
    };

    let mut borders = Vec::new();
    let (width, height) = img.dimensions();
    let b_threshold = args.brightness_diff;
    let hue_threshold = args.color_diff % 360;

    // Compares a pixel to the one on its right and below
    for y in 0..height - 1 {
        for x in 0..width - 1 {
            let current_pixel = img.get_pixel(x, y);
            let right_pixel = img.get_pixel(x + 1, y);
            let bottom_pixel = img.get_pixel(x, y + 1);

            // Single if statement - modify this to change detection strategy
            let is_border = match criteria {
                BorderCriteria::Color => {
                    // Color (hue) detection only
                    hue_difference(&current_pixel, &right_pixel) > hue_threshold
                        || hue_difference(&current_pixel, &bottom_pixel) > hue_threshold
                }
                BorderCriteria::Brightness => {
                    // Brightness detection only
                    brightness_difference(&current_pixel, &right_pixel) > b_threshold
                        || brightness_difference(&current_pixel, &bottom_pixel) > b_threshold
                }
                BorderCriteria::Both => {
                    // Both color and brightness detection (OR logic)
                    hue_difference(&current_pixel, &right_pixel) > hue_threshold
                        || hue_difference(&current_pixel, &bottom_pixel) > hue_threshold
                        || brightness_difference(&current_pixel, &right_pixel) > b_threshold
                        || brightness_difference(&current_pixel, &bottom_pixel) > b_threshold
                }
            };

            if is_border {
                borders.push((x, y));
            }
        }
    }

    borders
}

// Paints the borders of an image black
fn paint_borders(img: &mut image::RgbaImage, borders: Vec<(u32, u32)>, thickness: u32) {
    let half_t = thickness / 2;
    let width = img.width();
    let height = img.height();
    let black_pixel = image::Rgba([0, 0, 0, 255]);

    for (x, y) in borders {
        // Pre-calculate bounds
        let x_start = x.saturating_sub(half_t);
        let y_start = y.saturating_sub(half_t);
        let x_end = (x + half_t).min(width);
        let y_end = (y + half_t).min(height);

        // Direct iteration over valid coordinates
        for ny in y_start..y_end {
            for nx in x_start..x_end {
                img.put_pixel(nx, ny, black_pixel);
            }
        }
    }
}

// Resizes an image
pub fn resize(img: &mut RgbaImage, args: &mut Args) {
    let (orig_width, orig_height) = img.dimensions();
    if args.verbose {
        println!("Original dimensions {}x{}", orig_width, orig_height);
    }

    // Calculate dimensions once upfront
    let (target_width, target_height) = match (args.width_in_pixels, args.height_in_pixels) {
        (0, h) => {
            let ratio = h as f32 / orig_height as f32;
            (((orig_width as f32) * ratio) as u32, h)
        }
        (w, 0) => {
            let ratio = w as f32 / orig_width as f32;
            (w, ((orig_height as f32) * ratio) as u32)
        }
        (w, h) => (w, h),
    };

    args.width_in_pixels = target_width;
    args.height_in_pixels = target_height;

    // Resize the image
    *img = image::imageops::resize(
        img,
        target_width,
        target_height,
        image::imageops::FilterType::CatmullRom,
    );
}

// Saturates an image
pub fn saturate(img: &mut RgbaImage, args: &Args) {
    for pixel in img.pixels_mut() {
        let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
        let max = r.max(g).max(b);
        let factor = 255.0 / max as f32;

        // Only saturate if the pixel is bright enough
        if calculate_brightness(pixel) > args.midpoint_brightness {
            pixel[0] = (r as f32 * factor).round() as u8;
            pixel[1] = (g as f32 * factor).round() as u8;
            pixel[2] = (b as f32 * factor).round() as u8;
        } else {
            pixel[0] = (r as f32 / factor).round() as u8;
            pixel[1] = (g as f32 / factor).round() as u8;
            pixel[2] = (b as f32 / factor).round() as u8;
        }
    }
}

// Adds padding to an image
pub fn add_padding(img: &mut RgbaImage, args: &Args) {
    // Calculate dimensions
    let (img_width, img_height) = img.dimensions();
    let new_width = img_width + (args.padding_x * 2) as u32;
    let new_height = img_height + (args.padding_y * 2) as u32;
    let pixel_bytes = 4;

    // The pixels should be trasparent, so depending on the visible flag they are black or white
    let mut new_bytes =
        vec![if args.visible { 255 } else { 0 }; (new_width * new_height) as usize * pixel_bytes];
    let original_bytes = img.as_bytes();

    // Copy original image data with padding
    for y in 0..img_height {
        let src_start = (y * img_width) as usize * pixel_bytes;
        let src_end = src_start + (img_width as usize * pixel_bytes);
        let dst_start = ((y + args.padding_y as u32) * new_width + args.padding_x as u32) as usize
            * pixel_bytes;

        new_bytes[dst_start..dst_start + (img_width as usize * pixel_bytes)]
            .copy_from_slice(&original_bytes[src_start..src_end]);
    }

    *img = image::RgbaImage::from_raw(new_width, new_height, new_bytes)
        .expect("Failed to create padding image");

    if args.verbose {
        println!("Applied padding of {}x{}", args.padding_x, args.padding_y);
    }
}

pub fn center_image(img: &RgbaImage, args: &mut Args, font: &FontBitmap) {
    let (img_width, img_height) = img.dimensions();
    let num_blocks_x = ((img_width as usize + font.width - 1) / font.width) as u32;
    let num_blocks_y = ((img_height as usize + font.vertical_step - 1) / font.vertical_step) as u32;
    let target_width = num_blocks_x * font.width as u32;
    let target_height = num_blocks_y * font.vertical_step as u32;
    args.padding_x += ((target_width - img_width) / 2) as usize;
    args.padding_y += ((target_height - img_height) / 2) as usize;
}

pub fn grayscale(img: &mut RgbaImage) {
    // Convert to grayscale
    for pixel in img.pixels_mut() {
        let gray = calculate_brightness(pixel);
        let gray_value = (gray * 255.0).round() as u8;
        pixel[0] = gray_value;
        pixel[1] = gray_value;
        pixel[2] = gray_value;
    }

    // Find the brightest and darkest pixels
    // Only need to check one channel since it's grayscale
    let mut max_brightness = 0u8;
    let mut min_brightness = 255u8;
    for pixel in img.pixels() {
        max_brightness = max_brightness.max(pixel[0]);
        min_brightness = min_brightness.min(pixel[0]);
    }

    // Normalize the image based on the brightness range
    if max_brightness > min_brightness {
        let range = max_brightness - min_brightness;
        let factor = 255.0 / range as f32;
        for pixel in img.pixels_mut() {
            let normalized = ((pixel[0] - min_brightness) as f32 * factor).round() as u8;
            pixel[0] = normalized;
            pixel[1] = normalized;
            pixel[2] = normalized;
        }
    }
}

// Preprocesses an image to black and white
pub fn bw_filter(img: &mut RgbaImage, args: &Args) {
    // Convert the image to black and white applying a threshold
    for pixel in img.pixels_mut() {
        let pixel_brightness = calculate_brightness(&pixel);
        let value = if pixel_brightness > args.threshold {
            255
        } else {
            0
        };
        pixel[0] = value;
        pixel[1] = value;
        pixel[2] = value;
        pixel[3] = 255; // Keep alpha fully opaque
    }
}

// Applies the negative effect to an image
pub fn negative(img: &mut RgbaImage) {
    for pixel in img.pixels_mut() {
        let pixel_brightness = calculate_brightness(&pixel);
        let target_brightness = 1.0 - pixel_brightness;
        if pixel_brightness == 0.0 {
            pixel[0] = 255;
            pixel[1] = 255;
            pixel[2] = 255;
        } else {
            // Apply the negative effect (it is squared to make it more visible)
            let factor = (target_brightness / pixel_brightness).powf(2.0);
            pixel[0] = (pixel[0] as f32 * factor).round() as u8;
            pixel[1] = (pixel[1] as f32 * factor).round() as u8;
            pixel[2] = (pixel[2] as f32 * factor).round() as u8;
        }
    }
}

// Make transparent pixels visible
pub fn treat_transparent(img: &mut RgbaImage, args: &Args) {
    for pixel in img.pixels_mut() {
        if pixel[3] == 0 {
            if args.visible {
                pixel[0] = 255;
                pixel[1] = 255;
                pixel[2] = 255;
            } else {
                pixel[0] = 0;
                pixel[1] = 0;
                pixel[2] = 0;
            }
        }
        pixel[3] = 255;
    }
}
