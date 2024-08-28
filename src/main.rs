mod abc;

use image::{GenericImageView, Rgba};
use std::io;

fn calculate_brightness(pixel: Rgba<u8>) -> f32 {
    let r = pixel[0] as f32 / 255.0;
    let g = pixel[1] as f32 / 255.0;
    let b = pixel[2] as f32 / 255.0;

    let brightness = (0.299 * r + 0.587 * g + 0.114 * b).sqrt();
    brightness
}

fn match_group_with_letter(group: [[f32; 6]; 12]) -> char {
    let mut best_match = ' ';
    let mut best_match_value = f32::MIN;

    for (letter, letter_pixels) in abc::get_dict().iter() {
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

fn main() -> io::Result<()> {
    // Load the image
    let img = image::open("./Hacker5 (Personalizado) (1).png").unwrap();

    // Convert to grayscale if necessary
    let (width, height) = img.dimensions();

    // Calculate number of 6x12 groups
    let num_groups_x = (width + 5) / 6;
    let num_groups_y = (height + 11) / 12;

    println!("Image dimensions: {}x{}", width, height);
    println!("Number of 6x12 groups: {}x{}", num_groups_x, num_groups_y);

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
                }
            }
            print!("{}", match_group_with_letter(group));
        }
        println!();
    }

    Ok(())
}
