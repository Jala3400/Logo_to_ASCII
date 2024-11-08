use crate::{
    args::Args,
    proc_block::match_group_with_letter,
    proc_pixel::{brightness_difference, calc_custom_brightness, hue_difference},
    types::{Bitmap, FontBitmap},
};
use image::{DynamicImage, GenericImage, GenericImageView};
use imageproc::contrast::threshold;

pub fn convert_bitmap(bitmap: &Bitmap, font: &FontBitmap) {
    // Get the dimensions of the image
    let height = bitmap.height;
    let width = bitmap.width;

    // Calculate number of 8x16 groups
    let num_groups_x = (width + 7) / 8;
    let num_groups_y = (height + 15) / 16;

    println!("Image dimensions: {}x{}", width, height);
    println!("Number of 8x16 groups: {}x{}", num_groups_x, num_groups_y);

    let mut group = [[0f32; 8]; 16];
    let mut bright_pixels;
    let mut full_pixels;

    // Iterate over 8x16 groups
    for y in 0..num_groups_y as usize {
        for x in 0..num_groups_x as usize {
            bright_pixels = 0;
            full_pixels = 0;
            for by in 0..16 as usize {
                let iy = y * 16 + by;
                for bx in 0..8 as usize {
                    let ix = x * 8 + bx;
                    if iy < height && ix < width {
                        let pixel = bitmap.data[iy * width + ix];
                        group[by][bx] = pixel;
                        if pixel > -0.5 {
                            bright_pixels += 1;
                            full_pixels += (pixel >= bitmap.max_brightness) as usize;
                        }
                    } else {
                        group[by][bx] = -0.5;
                    }
                }
            }
            print!(
                "{}",
                if full_pixels == 128 {
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
    let mut bitmap = Vec::new();
    let mut max_brightness = -0.5f32;

    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            let brightness = calc_custom_brightness(&pixel, args.inverse, args.visible);
            bitmap.push(brightness); 
            max_brightness = max_brightness.max(brightness);
        }
    }

    Bitmap {
        data: bitmap,
        width: img.width() as usize,
        height: img.height() as usize,
        max_brightness,
    }
}

pub fn black_and_white(img: &DynamicImage, args: &Args) -> Bitmap {
    let gray_img = image::ImageBuffer::from_raw(
        img.width(),
        img.height(),
        img.to_luma_alpha8()
            .into_raw()
            .chunks(2)
            .map(|chunk| {
                if chunk[1] == 0 {
                    if args.visible {
                        255
                    } else {
                        0
                    }
                } else {
                    chunk[0]
                }
            })
            .collect::<Vec<u8>>(),
    )
    .unwrap();
    let bw = threshold(&gray_img, args.threshold);

    let mut bitmap = Vec::new();
    for y in 0..bw.height() {
        for x in 0..bw.width() {
            let pixel = bw.get_pixel(x, y);
            bitmap.push(if pixel[0] == 0 { -0.5 } else { 0.5 });
        }
    }
    Bitmap {
        data: bitmap,
        width: bw.width() as usize,
        height: bw.height() as usize,
        max_brightness: 0.5,
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
