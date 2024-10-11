use std::collections::HashMap;

use image::GenericImageView;

use crate::{
    args::Args,
    proc_block::{match_group_with_letter, process_block_brightness, process_block_color},
};

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
            process_block(&img, x, y, &args, &font);
        }
        println!();
    }
}

fn process_block(
    img: &image::DynamicImage,
    x: u32,
    y: u32,
    args: &Args,
    font: &HashMap<char, Vec<Vec<f32>>>,
) {
    let group = if args.color {
        process_block_color(img, x, y, args.inverse, args.block)
    } else {
        process_block_brightness(img, x, y, args.inverse)
    };

    let letter = match_group_with_letter(group, &font);
    print!("{}", letter);
}