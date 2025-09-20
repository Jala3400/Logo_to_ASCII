use crate::{
    args::Args,
    proc_pixel::calculate_brightness,
    types::{CharInfo, FontBitmap},
};
use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

/// Returns a FontBitmap with the characters and their brightness values
pub fn get_dict(args: &Args) -> FontBitmap {
    // Load or create an image
    let mut img;

    // The size of a character is 1x2 pixels
    let width = 8usize;
    let height = width * 2;

    // Load a font
    let font: Font<'_>;
    if let Some(font_path) = args.font.as_ref() {
        let font_data =
            std::fs::read(&font_path).expect(&format!("Failed to read font file {font_path}"));
        font = Font::try_from_vec(font_data).expect("Failed to load font");
    } else {
        font = Font::try_from_bytes(include_bytes!("../fonts/UbuntuMono-Regular.ttf")).unwrap();
    }

    // Define text properties
    let scale = Scale::uniform(height as f32);
    let color = Rgba([255, 255, 255, 255]);

    // Create a vector of characters directly from the input string
    let characters: Vec<char> = args.chars.chars().collect();

    let mut final_font = FontBitmap { data: Vec::new() };

    // Create a character for each character in the input string
    for i in 0..characters.len() {
        // Create an image and then print the character on it
        let mut character: [f32; 8 * 16] = [0.0; 8 * 16];
        img = RgbaImage::new(width as u32, height as u32);
        let character_string = characters[i].to_string();
        draw_text_mut(&mut img, color, 0, 0, scale, &font, &character_string);

        let mut bright_blocks = 0;

        // Get the color of each pixel from the image
        for y in 0..img.height() {
            for x in 0..img.width() {
                let pixel = img.get_pixel(x, y);
                let brightness = calculate_brightness(pixel) - 0.5;
                if brightness > 0.0 {
                    bright_blocks += 1;
                }
                character[y as usize * width + x as usize] = brightness;
            }
        }

        // Add the character in the final font
        let char_info = CharInfo {
            char: characters[i],
            data: character,
            min: bright_blocks / 2,
        };
        final_font.insert_ord(char_info);
    }

    // Print all characters in the final font (ordered)
    if args.verbose {
        print!("Characters: ");
        for char_info in &final_font.data {
            print!("{}", char_info.char);
        }
        println!();
    }

    final_font
}
