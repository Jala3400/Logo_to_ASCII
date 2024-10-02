use std::collections::HashMap;
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

use image::{Rgba, RgbaImage};

pub fn get_dict8x16() -> HashMap<char, Vec<Vec<f32>>> {
    // Load or create an image
    let mut img ;

    // Load a font
    let font = Font::try_from_bytes(include_bytes!("C:/Windows/Fonts/Consola.ttf")).unwrap();

    // Define text properties
    let mut text;
    let scale = Scale::uniform(16.0);
    let color = Rgba([255, 255, 255, 255]);

    // Create an array to store characters
    let mut characters = Vec::new();

    // Extract characters from the font
    for c in 32..=126 as u8 {
        // ASCII printable characters
        print!("{}", c as char);
        characters.push(c as char);
    }
    println!("");

    let mut char_map: HashMap<char, Vec<Vec<f32>>> = HashMap::new();
    for i in 0..characters.len() {
        let mut character = Vec::new();
        img = RgbaImage::new(8, 16);
        let character_string = characters[i].to_string();
        text = &character_string;
        draw_text_mut(&mut img, color, 0, 0, scale, &font, text);

        // Get the color of each pixel from the image
        for y in 0..img.height() {
            let mut row = Vec::new();
            for x in 0..img.width() {
                let pixel = img.get_pixel(x, y);
                row.push(calculate_brightness(pixel));
            }
            character.push(row);
        }

        char_map.insert(characters[i], character);
    }

    char_map
}

pub fn calculate_brightness(pixel: &Rgba<u8>) -> f32 {
    let r = pixel[0] as f32 / 255.0;
    let g = pixel[1] as f32 / 255.0;
    let b = pixel[2] as f32 / 255.0;

    let brightness = (0.299 * r + 0.587 * g + 0.114 * b).sqrt();
    brightness
}