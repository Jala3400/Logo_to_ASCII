use crate::{
    args::Args,
    proc_pixel::{brightness_difference, calculate_brightness, hue_difference},
};
use image::{EncodableLayout, RgbaImage};

// Detects the borders of an image and paints them black
pub fn borders_image(img: &mut RgbaImage, args: &Args) {
    // Get the borders (difference color or brightness)
    let borders = if args.color_borders {
        detect_color_borders(&img, args.difference)
    } else {
        detect_borders(&img, args.difference as f32 / 360.0)
    };

    // Paint the borders
    paint_borders(img, borders, args);
}

// Detects the borders of an image using brightness
fn detect_borders(img: &image::RgbaImage, threshold: f32) -> Vec<(u32, u32)> {
    let mut borders = Vec::new();
    let (width, height) = img.dimensions();

    // Compares a pixel to the one on its right and below
    for y in 0..height - 1 {
        for x in 0..width - 1 {
            let current_pixel = img.get_pixel(x, y);
            let right_pixel = img.get_pixel(x + 1, y);
            let bottom_pixel = img.get_pixel(x, y + 1);

            if brightness_difference(&current_pixel, &right_pixel) > threshold
                || brightness_difference(&current_pixel, &bottom_pixel) > threshold
            {
                borders.push((x, y));
            }
        }
    }

    borders
}

// Detects the borders of an image using color
fn detect_color_borders(img: &image::RgbaImage, threshold: u16) -> Vec<(u32, u32)> {
    let mut borders = Vec::new();
    let (width, height) = img.dimensions();

    // Compares a pixel to the one on its right and below
    for y in 0..height - 1 {
        for x in 0..width - 1 {
            let current_pixel = img.get_pixel(x, y);
            let right_pixel = img.get_pixel(x + 1, y);
            let bottom_pixel = img.get_pixel(x, y + 1);

            if hue_difference(&current_pixel, &right_pixel) > threshold
                || hue_difference(&current_pixel, &bottom_pixel) > threshold
            {
                borders.push((x, y));
            }
        }
    }

    borders
}

// Paints the borders of an image black
fn paint_borders(img: &mut image::RgbaImage, borders: Vec<(u32, u32)>, args: &Args) {
    // Precalculate needed values
    let thickness = if args.border == 0 { 8 } else { args.border };
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
    let (target_width, target_height) = match (args.actual_width, args.actual_height) {
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

    args.actual_width = target_width;
    args.actual_height = target_height;

    // Resize the image
    *img = image::imageops::resize(
        img,
        target_width,
        target_height,
        image::imageops::FilterType::Lanczos3,
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

// Adds an offset to an image
pub fn add_offset(img: &mut RgbaImage, args: &Args) {
    // Calculate dimensions
    let (img_width, img_height) = img.dimensions();
    let new_width = img_width + args.offsetx as u32;
    let new_height = img_height + args.offsety as u32;
    let pixel_bytes = 4;

    let mut new_bytes = vec![0; (new_width * new_height) as usize * pixel_bytes];
    let original_bytes = img.as_bytes();

    // Copy original image data with offset
    for y in 0..img_height {
        let src_start = (y * img_width) as usize * pixel_bytes;
        let src_end = src_start + (img_width as usize * pixel_bytes);
        let dst_start =
            ((y + args.offsety as u32) * new_width + args.offsetx as u32) as usize * pixel_bytes;

        new_bytes[dst_start..dst_start + (img_width as usize * pixel_bytes)]
            .copy_from_slice(&original_bytes[src_start..src_end]);
    }

    *img = image::RgbaImage::from_raw(new_width, new_height, new_bytes)
        .expect("Failed to create offset image");
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
