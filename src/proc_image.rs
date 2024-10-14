use crate::{
    args::Args,
    proc_block::match_group_with_letter,
    proc_pixel::{brightness_difference, calc_custom_brightness, hue_difference},
};
use image::{DynamicImage, GenericImage, GenericImageView};
use imageproc::contrast::threshold;
use std::collections::HashMap;

pub fn convert_bitmap(bitmap: Vec<Vec<f32>>, font: HashMap<char, Vec<Vec<f32>>>) {
    // Get the dimensions of the image
    let height = bitmap.len();
    let width = bitmap[0].len();

    // Calculate number of 8x16 groups
    let num_groups_x = (width + 7) / 8;
    let num_groups_y = (height + 15) / 16;

    println!("Image dimensions: {}x{}", width, height);
    println!("Number of 8x16 groups: {}x{}", num_groups_x, num_groups_y);

    // Iterate over 8x16 groups
    for y in 0..num_groups_y {
        for x in 0..num_groups_x {
            let mut group = [[0f32; 8]; 16];
            for by in 0..16 {
                for bx in 0..8 {
                    let iy = y * 16 + by;
                    let ix = x * 8 + bx;
                    if iy < height && ix < width {
                        group[by][bx] = bitmap[iy][ix];
                    }
                }
            }
            print!("{}", match_group_with_letter(group, &font));
        }
        println!();
    }
}

pub fn get_bitmap(img: DynamicImage, args: &Args) -> Vec<Vec<f32>> {
    let mut bitmap = Vec::new();

    for y in 0..img.height() {
        let mut row = Vec::new();
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            row.push(calc_custom_brightness(pixel, args.inverse));
        }
        bitmap.push(row);
    }

    bitmap
}

pub fn to_black_and_white(img: DynamicImage, args: &Args) -> DynamicImage {
    let gray_img = img.to_luma8();
    image::DynamicImage::ImageLuma8(threshold(&gray_img, args.threshold))
}

pub fn borders_image(img: DynamicImage, args: &Args) -> DynamicImage {
    let bw_image = to_black_and_white(img, args);
    let borders = detect_borders(&bw_image, args.difference as f32 / 360 as f32);
    let mut new_img = DynamicImage::new_luma8(bw_image.width(), bw_image.height());
    for (x, y) in borders {
        for dy in 0..args.border {
            for dx in 0..args.border {
                let nx = x as i32 + dx as i32 - (args.border / 2) as i32;
                let ny = y as i32 + dy as i32 - (args.border / 2) as i32;
                if nx >= 0
                    && ny >= 0
                    && nx < bw_image.width() as i32
                    && ny < bw_image.height() as i32
                {
                    new_img.put_pixel(nx as u32, ny as u32, image::Rgba([255, 255, 255, 255]));
                }
            }
        }
    }
    new_img
}

pub fn borders_image_color(img: DynamicImage, args: &Args) -> DynamicImage {
    let borders = detect_color_borders(&img, args.difference);
    let mut new_img = img.clone();
    let thickness = if args.border == 0 { 1 } else { args.border };
    for (x, y) in borders {
        for dy in 0..thickness {
            for dx in 0..thickness {
                let nx = x as i32 + dx as i32 - (args.border / 2) as i32;
                let ny = y as i32 + dy as i32 - (args.border / 2) as i32;
                if nx >= 0 && ny >= 0 && nx < img.width() as i32 && ny < img.height() as i32 {
                    new_img.put_pixel(nx as u32, ny as u32, image::Rgba([0, 0, 0, 255]));
                }
            }
        }
    }
    new_img
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
