use crate::{
    args::Args,
    proc_block::match_group_with_letter,
    proc_pixel::calc_custom_brightness,
    types::{Bitmap, FontBitmap},
};
use image::{DynamicImage, GenericImageView};

pub fn convert_bitmap(bitmap: &Bitmap, font: &FontBitmap, args: &Args) {
    let height = bitmap.height;
    let width = bitmap.width;

    let num_groups_x = (width + 7) / 8;
    let num_groups_y = (height + 15) / 16;

    println!("Image dimensions: {}x{}", width, height);
    println!("Number of 8x16 groups: {}x{}", num_groups_x, num_groups_y);

    let mut group = [0f32; 8 * 16];
    let mut bright_pixels;
    let mut full_pixels;

    for y in 0..num_groups_y {
        for x in 0..num_groups_x {
            bright_pixels = 0;
            full_pixels = 0;
            for by in 0..16 {
                let iy = y * 16 + by;
                for bx in 0..8 {
                    let ix = x * 8 + bx;
                    let cords = iy * width + ix;
                    let cords_block = by * 8 + bx;
                    if iy < height && ix < width {
                        let pixel = bitmap.data[cords];
                        group[cords_block] = pixel;
                        if pixel > -args.midpoint_brightness {
                            bright_pixels += 1;
                            full_pixels += (pixel >= bitmap.max_brightness) as usize;
                        }
                    } else {
                        group[cords_block] = -args.midpoint_brightness;
                    }
                }
            }
            print!(
                "{}",
                if full_pixels == 16 * 8 {
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
