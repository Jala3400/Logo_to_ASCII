use crate::{
    args::Args, proc_block::match_group_with_letter, proc_pixel::calc_custom_brightness,
    types::FontBitmap,
};
use enable_ansi_support::enable_ansi_support;
use image::{DynamicImage, GenericImageView};

pub fn convert_image(img: &DynamicImage, font: &FontBitmap, args: &Args) {
    enable_ansi_support().unwrap();
    let height = img.height() as usize;
    let width = img.width() as usize;

    let num_groups_x = (width + 7) / 8;
    let num_groups_y = (height + 15) / 16;

    println!("Image dimensions: {}x{}", width, height);
    println!("Number of characters: {}x{}", num_groups_x, num_groups_y);

    let mut group = [0f32; 8 * 16];
    let mut bright_pixels;
    let mut full_pixels;
    let mut r: usize;
    let mut g: usize;
    let mut b: usize;

    for y in 0..num_groups_y {
        for x in 0..num_groups_x {
            bright_pixels = 0;
            full_pixels = 0;
            r = 0;
            g = 0;
            b = 0;
            for by in 0..16 {
                let iy = y * 16 + by;
                for bx in 0..8 {
                    let ix = x * 8 + bx;
                    let cords_block = by * 8 + bx;
                    if iy < height && ix < width {
                        let pixel = img.get_pixel(ix as u32, iy as u32);
                        group[cords_block] = calc_custom_brightness(&pixel, args);
                        if group[cords_block] > -args.midpoint_brightness {
                            bright_pixels += 1;
                            full_pixels +=
                                (group[cords_block] >= 1.0 - args.midpoint_brightness) as usize;
                            r += pixel[0] as usize;
                            g += pixel[1] as usize;
                            b += pixel[2] as usize;
                        }
                    } else {
                        group[cords_block] = if args.visible {
                            1.0 - args.midpoint_brightness
                        } else {
                            -args.midpoint_brightness
                        }
                    }
                }
            }

            print!(
                "{}{}",
                if args.text_color && bright_pixels > 0 {
                    r /= bright_pixels;
                    g /= bright_pixels;
                    b /= bright_pixels;
                    format!("\x1b[38;2;{};{};{}m", r, g, b)
                } else {
                    String::new()
                },
                if full_pixels == 16 * 8 {
                    font.data.last().unwrap().char
                } else {
                    match_group_with_letter(&group, font, bright_pixels)
                }
            );
        }
        println!();
    }
    print!("\x1b[0m");
}
