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

    // Iterate over 8x16 groups
    for y in 0..num_groups_y as usize {
        for x in 0..num_groups_x as usize {
            let mut group = [[0f32; 8]; 16];
            let mut bright_blocks = 0;
            for by in 0..16 as usize {
                for bx in 0..8 as usize {
                    let iy = y * 16 + by;
                    let ix = x * 8 + bx;
                    if iy < height && ix < width {
                        group[by][bx] = bitmap.data[iy * width + ix];
                        if group[by][bx] > -0.5 {
                            bright_blocks += 1;
                        }
                    } else if iy >= height || ix >= width {
                        group[by][bx] = -0.5;
                    }
                }
            }
            print!("{}", match_group_with_letter(group, &font, bright_blocks));
        }
        println!();
    }
}

pub fn get_bitmap(img: &DynamicImage, args: &Args) -> Bitmap {
    let mut bitmap = Vec::new();

    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            bitmap.push(calc_custom_brightness(&pixel, args.inverse, args.visible));
        }
    }

    Bitmap {
        data: bitmap,
        width: img.width() as usize,
        height: img.height() as usize,
    }
}

pub fn black_and_white(img: &DynamicImage, args: &Args) -> Bitmap {
    let gray_img = img.to_luma8();
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
    }
}

pub fn borders_image(mut img: &mut DynamicImage, args: &Args) {
    let borders = if args.color {
        detect_color_borders(&img, args.difference)
    } else {
        detect_borders(&img, args.difference as f32 / 360.0)
    };
    paint_borders(&mut img, borders, args);
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

            if hue_difference(&current_pixel, &right_pixel) > threshold as f32
                || hue_difference(&current_pixel, &bottom_pixel) > threshold as f32
            {
                borders.push((x, y));
            }
        }
    }

    borders
}

fn paint_borders(img: &mut image::DynamicImage, borders: Vec<(u32, u32)>, args: &Args) {
    let thickness = if args.border == 0 { 8 } else { args.border };
    for (x, y) in borders {
        for dy in 0..thickness {
            for dx in 0..thickness {
                let nx = x as i32 + dx as i32 - (thickness / 2) as i32;
                let ny = y as i32 + dy as i32 - (thickness / 2) as i32;
                if nx >= 0 && ny >= 0 && nx < img.width() as i32 && ny < img.height() as i32 {
                    img.put_pixel(nx as u32, ny as u32, image::Rgba([0, 0, 0, 255]));
                }
            }
        }
    }
}
