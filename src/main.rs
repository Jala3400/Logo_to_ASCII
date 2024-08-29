mod abc;

use image::{GenericImageView, Rgba};
use std::io;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of the image to process
    #[arg(short, long)]
    path: String,

    /// width of the letters in pixels
    #[arg(short, long, default_value_t = 6)]
    width: u8,

    /// Inverse the colors of the image
    #[arg(short, long, default_value_t = false)]
    inverse: bool,
}

fn calculate_brightness(pixel: Rgba<u8>) -> f32 {
    let r = pixel[0] as f32 / 255.0;
    let g = pixel[1] as f32 / 255.0;
    let b = pixel[2] as f32 / 255.0;

    let brightness = (0.299 * r + 0.587 * g + 0.114 * b).sqrt();
    brightness
}

fn match_group_with_letter_8x16(group: [[f32; 8]; 16]) -> char {
    let mut best_match = ' ';
    let mut best_match_value = f32::MIN;

    for (letter, letter_pixels) in abc::get_dict8x16().iter() {
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
fn match_group_with_letter_6x12(group: [[f32; 6]; 12]) -> char {
    let mut best_match = ' ';
    let mut best_match_value = f32::MIN;

    for (letter, letter_pixels) in abc::get_dict6x12().iter() {
        let mut match_value = 0.0;
        for y in 0..12 {
            for x in 0..6 {
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

fn process_8x16(img: image::DynamicImage, args: Args) {
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
            // Process pixels in the current group

            let mut group = [[0.0; 8]; 16];

            for pixel_y in 0..16 {
                for pixel_x in 0..8 {
                    if (x * 8 + pixel_x) >= width || (y * 16 + pixel_y) >= height {
                        group[pixel_y as usize][pixel_x as usize] = -0.5;
                        continue;
                    }
                    let pixel = img.get_pixel(x * 8 + pixel_x, y * 16 + pixel_y);
                    let brightness = calculate_brightness(pixel);
                    group[pixel_y as usize][pixel_x as usize] = brightness - 0.5;

                    if args.inverse {
                        group[pixel_y as usize][pixel_x as usize] *= -1.0;
                    }
                }
            }
            print!("{}", match_group_with_letter_8x16(group));
        }
        println!();
    }
}

fn process_6x12(img: image::DynamicImage, args: Args) {
    // Get the dimensions of the image
    let (width, height) = img.dimensions();

    // Calculate number of 8x16 groups
    let num_groups_x = (width + 5) / 6;
    let num_groups_y = (height + 11) / 12;

    // Iterate over 6x12 groups
    for y in 0..num_groups_y {
        for x in 0..num_groups_x {
            // Process pixels in the current group

            let mut group = [[0.0; 6]; 12];

            for pixel_y in 0..12 {
                for pixel_x in 0..6 {
                    if (x * 6 + pixel_x) >= width || (y * 12 + pixel_y) >= height {
                        group[pixel_y as usize][pixel_x as usize] = -0.5;
                        continue;
                    }
                    let pixel = img.get_pixel(x * 6 + pixel_x, y * 12 + pixel_y);
                    let brightness = calculate_brightness(pixel);
                    group[pixel_y as usize][pixel_x as usize] = brightness - 0.5;

                    if args.inverse {
                        group[pixel_y as usize][pixel_x as usize] *= -1.0;
                    }
                }
            }
            print!("{}", match_group_with_letter_6x12(group));
        }
        println!();
    }
}

fn main() -> io::Result<()> {
    let args: Args = Args::parse();

    // Load the image
    let img = image::open(&args.path).unwrap();

    match args.width {
        6 => {
            process_6x12(img, args);
        }
        8 => {
            process_8x16(img, args);
        }
        _ => {
            println!("Invalid width, 6 used by default");
            process_6x12(img, args);
        }
    }

    Ok(())
}
