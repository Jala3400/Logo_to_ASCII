use crate::{
    proc_pixel::calculate_brightness,
    types::{CharInfo, FontBitmap},
};
use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

pub fn get_dict8x16(font_path: &Option<String>, chars: &str) -> FontBitmap {
    // Load or create an image
    let mut img;

    let width = 8;
    let height = 16;

    // Load a font
    let font: Font<'_>;
    if let Some(font_path) = font_path {
        let font_data =
            std::fs::read(&font_path).expect(&format!("Failed to read font file {font_path}"));
        font = Font::try_from_vec(font_data).expect("Failed to load font");
    } else {
        font = Font::try_from_bytes(include_bytes!("C:/Windows/Fonts/Consola.ttf")).unwrap();
    }

    // Define text properties
    let mut text;
    let scale = Scale::uniform(height as f32);
    let color = Rgba([255, 255, 255, 255]);

    // Create an array to store characters
    let mut characters = Vec::new();

    // Extract characters from the font
    print!("Characters: ");
    for c in chars.chars() {
        // ASCII printable characters
        print!("{}", c);
        characters.push(c);
    }
    println!("");

    let mut final_font = FontBitmap {
        data: Vec::new(),
        width: width as usize,
        height: height as usize,
    };
    for i in 0..characters.len() {
        let mut character = Vec::new();
        img = RgbaImage::new(width, height);
        let character_string = characters[i].to_string();
        text = &character_string;
        draw_text_mut(&mut img, color, 0, 0, scale, &font, text);

        let mut bright_blocks = 0;

        // Get the color of each pixel from the image
        for y in 0..img.height() {
            for x in 0..img.width() {
                let pixel = img.get_pixel(x, y);
                let brightness = calculate_brightness(pixel);
                if brightness > 0.0 {
                    bright_blocks += 1;
                }
                character.push(brightness - 0.5);
            }
        }

        let char_info = CharInfo {
            char: characters[i],
            data: character,
            min: bright_blocks / 2,
        };
        final_font.insert_ord(char_info);
    }

    final_font
}
