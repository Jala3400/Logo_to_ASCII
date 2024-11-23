use crate::{
    args::Args,
    proc_block::match_group_with_letter,
    proc_pixel::{brightness_difference, calc_custom_brightness, hue_difference},
    types::{Bitmap, FontBitmap},
};
use image::{DynamicImage, GenericImage, GenericImageView};

pub fn convert_bitmap(bitmap: &Bitmap, font: &FontBitmap, args: &Args) {
    // Get the dimensions of the image
    let height = bitmap.height;
    let width = bitmap.width;

    let block_width = font.width;
    let block_height = font.height;

    // Calculate number of groups
    let num_groups_x = (width + block_width - 1) / block_width;
    let num_groups_y = (height + block_height - 1) / block_height;

    println!("Image dimensions: {}x{}", width, height);
    println!(
        "Number of {}x{} groups: {}x{}",
        block_width, block_height, num_groups_x, num_groups_y
    );

    let mut group = Vec::with_capacity(block_height * block_width);
    let mut bright_pixels;
    let mut full_pixels;

    // Iterate over groups
    for y in 0..num_groups_y as usize {
        for x in 0..num_groups_x as usize {
            group.clear();
            bright_pixels = 0;
            full_pixels = 0;
            for by in 0..block_height as usize {
                let iy = y * block_height + by;
                for bx in 0..block_width as usize {
                    let ix = x * block_width + bx;
                    let cords = iy * width + ix;
                    if iy < height && ix < width {
                        let pixel = bitmap.data[cords];
                        group.push(pixel);
                        if pixel > -args.midpoint_brightness {
                            bright_pixels += 1;
                            full_pixels += (pixel >= bitmap.max_brightness) as usize;
                        }
                    } else {
                        group.push(-args.midpoint_brightness);
                    }
                }
            }
            print!(
                "{}",
                if full_pixels == block_height * block_width {
                    font.data.last().unwrap().char
                } else {
                    match_group_with_letter(&group, font, bright_pixels)
                }
            );
        }
        println!();
    }
}

pub fn get_bitmap(img: &DynamicImage, args: &Args) -> Bitmap {
    let width = img.width() as usize;
    let height = img.height() as usize;

    // Pre-allocate vector with exact capacity
    let mut bitmap = Vec::with_capacity(width * height);
    let mut max_brightness = -args.midpoint_brightness;

    // Process all pixels in one pass
    bitmap.extend(img.pixels().map(|pixel| {
        let brightness = calc_custom_brightness(&pixel.2, args);
        max_brightness = max_brightness.max(brightness);
        brightness
    }));

    Bitmap {
        data: bitmap,
        width,
        height,
        max_brightness,
    }
}

pub fn black_and_white(img: &DynamicImage, args: &Args) -> Bitmap {
    let width = img.width() as usize;
    let height = img.height() as usize;
    let luma_alpha = img.to_luma_alpha8();
    let raw_data = luma_alpha.as_raw();

    // Preallocate bitmap with exact capacity
    let mut bitmap = Vec::with_capacity(width * height);

    // Process pixels in chunks of 2 (luma + alpha)
    for chunk in raw_data.chunks_exact(2) {
        let value = if chunk[1] == 0 {
            if args.visible {
                1.0 - args.midpoint_brightness
            } else {
                -args.midpoint_brightness
            }
        } else {
            let threshold_check = chunk[0] > args.threshold;
            if threshold_check == !args.inverse {
                1.0 - args.midpoint_brightness
            } else {
                -args.midpoint_brightness
            }
        };
        bitmap.push(value);
    }

    Bitmap {
        data: bitmap,
        width,
        height,
        max_brightness: 1.0 - args.midpoint_brightness,
    }
}

pub fn borders_image(img: &mut DynamicImage, args: &Args) {
    let borders = if args.color {
        detect_color_borders(&img, args.difference)
    } else {
        detect_borders(&img, args.difference as f32 / 360.0)
    };
    paint_borders(img, borders, args);
}

fn detect_borders(img: &image::DynamicImage, threshold: f32) -> Vec<(u32, u32)> {
    let mut borders = Vec::new();
    let (width, height) = img.dimensions();

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

fn detect_color_borders(img: &image::DynamicImage, threshold: u16) -> Vec<(u32, u32)> {
    let mut borders = Vec::new();
    let (width, height) = img.dimensions();

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

fn paint_borders(img: &mut image::DynamicImage, borders: Vec<(u32, u32)>, args: &Args) {
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
