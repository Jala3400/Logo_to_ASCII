mod abc;

use abc::calculate_brightness;
use image::GenericImageView;
use std::{collections::HashMap, io};

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of the image to process
    #[arg(short, long)]
    path: String,

    /// Path of the font to use (optional)
    #[arg(short, long)]
    font: Option<String>,

    /// Inverse the colors of the image
    #[arg(short, long, default_value_t = false)]
    inverse: bool,
}

fn match_group_with_letter(group: [[f32; 8]; 16], font: &HashMap<char, Vec<Vec<f32>>>) -> char {
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

fn process_block(img: image::DynamicImage, args: Args, font: HashMap<char, Vec<Vec<f32>>>) {
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
                    let brightness = calculate_brightness(&pixel);
                    group[pixel_y as usize][pixel_x as usize] = brightness - 0.5;

                    if args.inverse {
                        group[pixel_y as usize][pixel_x as usize] *= -1.0;
                    }
                }
            }
            print!("{}", match_group_with_letter(group, &font));
        }
        println!();
    }
}

fn main() -> io::Result<()> {
    let args: Args = Args::parse();

    // Load the image
    let img = image::open(&args.path).unwrap();

    let font = abc::get_dict8x16(&args.font);

    process_block(img, args, font);

    Ok(())
}