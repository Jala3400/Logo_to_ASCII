use image::GenericImageView;
use std::collections::HashMap;

use crate::{
    calc_pixel::{calc_custom_brightness, calc_hue},
    Args,
};

pub fn match_group_with_letter(group: [[f32; 8]; 16], font: &HashMap<char, Vec<Vec<f32>>>) -> char {
    let mut best_match = ' ';
    let mut best_match_value = f32::MIN;

    for (letter, letter_pixels) in font.iter() {
        let mut match_value = 0.0;
        for y in 0..16 {
            for x in 0..8 {
                match_value += group[y][x] * letter_pixels[y][x] as f32;
            }
        }

        if match_value > best_match_value {
            best_match_value = match_value;
            best_match = *letter;
        }
    }

    best_match
}

pub fn convert_image(img: image::DynamicImage, args: Args, font: HashMap<char, Vec<Vec<f32>>>) {
    // Get the dimensions of the image
    let (width, height) = img.dimensions();

    // Calculate number of 8x16 groups
    let num_groups_x = (width + 7) / 8;
    let num_groups_y = (height + 15) / 16;

    println!("Image dimensions: {}x{}", width, height);
    println!("Number of 8x16 groups: {}x{}", num_groups_x, num_groups_y);

    // Iterate over 8x16 groups
    for y in 0..num_groups_y {
        for x in 0..num_groups_x {
            let group = if args.color {
                process_block_color(&img, x, y, args.inverse)
            } else {
                process_block(&img, x, y, args.inverse)
            };
            print!("{}", match_group_with_letter(group, &font));
        }
        println!();
    }
}

fn process_block(img: &image::DynamicImage, x: u32, y: u32, inverse: bool) -> [[f32; 8]; 16] {
    let mut group: [[f32; 8]; 16] = [[0.0; 8]; 16];

    for pixel_y in 0..16 {
        for pixel_x in 0..8 {
            if (x * 8 + pixel_x) >= img.width() || (y * 16 + pixel_y) >= img.height() {
                continue;
            }
            let pixel = img.get_pixel(x * 8 + pixel_x, y * 16 + pixel_y);
            group[pixel_y as usize][pixel_x as usize] = calc_custom_brightness(pixel, inverse);
        }
    }

    group
}

fn process_block_color(img: &image::DynamicImage, x: u32, y: u32, inverse: bool) -> [[f32; 8]; 16] {
    let mut hue_count: HashMap<u8, u32> = HashMap::new();
    let mut group: [[f32; 8]; 16] = [[0.0; 8]; 16];

    // Count the frequency of each hue in the block
    for pixel_y in 0..16 {
        for pixel_x in 0..8 {
            if (x * 8 + pixel_x) >= img.width() || (y * 16 + pixel_y) >= img.height() {
                continue;
            }
            let pixel = img.get_pixel(x * 8 + pixel_x, y * 16 + pixel_y);
            let hue = calc_hue(pixel);
            *hue_count.entry(hue).or_insert(0) += 1;
        }
    }

    // Find the most frequent hue
    let most_frequent_hue = hue_count
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(hue, _)| hue)
        .unwrap_or(0);

    // Set the brightness of pixels with the most frequent hue
    for pixel_y in 0..16 {
        for pixel_x in 0..8 {
            if (x * 8 + pixel_x) >= img.width() || (y * 16 + pixel_y) >= img.height() {
                continue;
            }
            let pixel = img.get_pixel(x * 8 + pixel_x, y * 16 + pixel_y);
            let hue = calc_hue(pixel);
            if hue == most_frequent_hue {
                group[pixel_y as usize][pixel_x as usize] = calc_custom_brightness(pixel, inverse);
            } else {
                group[pixel_y as usize][pixel_x as usize] = -0.5 * if inverse { -1.0 } else { 1.0 };
            }
        }
    }

    group
}