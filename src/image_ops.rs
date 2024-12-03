use crate::{
    args::Args,
    proc_pixel::{brightness_difference, hue_difference},
    types::Bitmap,
};
use image::{DynamicImage, GenericImage, GenericImageView};

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

pub fn resize(img: &mut DynamicImage, args: &mut Args) {
    let (orig_width, orig_height) = img.dimensions();
    println!("Original dimensions {}x{}", orig_width, orig_height);

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

    *img = img.resize_exact(
        target_width,
        target_height,
        image::imageops::FilterType::Lanczos3,
    );
}
