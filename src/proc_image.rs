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

pub fn black_and_white(img: &DynamicImage, args: &Args) -> Bitmap {
    let width = img.width() as usize;
    let height = img.height() as usize;
    let luma_alpha = img.to_luma_alpha8();
    let raw_data = luma_alpha.as_raw();

    // Calculate total size including offset
    let total_width = width + args.offsetx as usize;
    let total_height = height + args.offsety as usize;
    let total_size = total_width * total_height;

    // Preallocate bitmap with total size
    let mut bitmap = Vec::with_capacity(total_size);

    // Fill offset rows at the top
    bitmap.extend(
        std::iter::repeat(-args.midpoint_brightness).take(total_width * args.offsety as usize),
    );

    // Process each row with offset
    for y in 0..height {
        // Add left offset
        bitmap.extend(std::iter::repeat(-args.midpoint_brightness).take(args.offsetx as usize));

        // Process pixels in chunks of 2 (luma + alpha)
        let row_start = y * width * 2;
        let row_end = row_start + width * 2;
        for chunk in raw_data[row_start..row_end].chunks_exact(2) {
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
    }

    Bitmap {
        data: bitmap,
        width: width + args.offsetx as usize,
        height: height + args.offsety as usize,
        max_brightness: 1.0 - args.midpoint_brightness,
    }
}

pub fn get_bitmap(img: &DynamicImage, args: &Args) -> Bitmap {
    let width = img.width() as usize;
    let height = img.height() as usize;
    let total_width = width + args.offsetx as usize;
    let total_height = height + args.offsety as usize;

    let off_value = if args.visible {
        1.0 - args.midpoint_brightness
    } else {
        -args.midpoint_brightness
    };

    // Pre-allocate vector with exact capacity
    let mut bitmap = Vec::with_capacity(total_width * total_height);
    let mut max_brightness = -args.midpoint_brightness;

    // Fill with background value up to offset
    bitmap.extend(std::iter::repeat(off_value).take((args.offsety as usize) * total_width));

    // Process image rows with offset
    for y in 0..height {
        // Add left offset
        bitmap.extend(std::iter::repeat(off_value).take(args.offsetx as usize));

        // Add image pixels for this row
        let row_start = y * width;
        bitmap.extend(img.pixels().skip(row_start).take(width).map(|pixel| {
            let brightness = calc_custom_brightness(&pixel.2, args);
            max_brightness = max_brightness.max(brightness);
            brightness
        }));
    }

    Bitmap {
        data: bitmap,
        width: total_width,
        height: total_height,
        max_brightness,
    }
}
