use crate::{
    args::Args,
    proc_pixel::calculate_brightness,
    types::{CharInfo, FontBitmap},
};
use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

pub fn get_dict(args: &Args) -> FontBitmap {
    // Load or create an image
    let mut img;

    let width = args.width;
    let height = args.width * 2;

    // Load a font
    let font: Font<'_>;
    if let Some(font_path) = args.font.as_ref() {
        let font_data =
            std::fs::read(&font_path).expect(&format!("Failed to read font file {font_path}"));
        font = Font::try_from_vec(font_data).expect("Failed to load font");
    } else {
        font = Font::try_from_bytes(include_bytes!("C:/Windows/Fonts/Consola.ttf")).unwrap();
    }

    // Define text properties
    let scale = Scale::uniform(height as f32);
    let color = Rgba([255, 255, 255, 255]);

    // Create a vector of characters directly from the input string
    let characters: Vec<char> = args.chars.chars().collect();

    let mut final_font = FontBitmap {
        data: Vec::new(),
        width: width as usize,
        height: height as usize,
    };

    for i in 0..characters.len() {
        let mut character = Vec::new();
        img = RgbaImage::new(width, height);
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
                character.push(brightness);
            }
        }

        let char_info = CharInfo {
            char: characters[i],
            data: character,
            min: bright_blocks / 2,
        };
        final_font.insert_ord(char_info);
    }

    // Print all characters in the final font (ordered)
    print!("Characters: ");
    for char_info in &final_font.data {
        print!("{}", char_info.char);
    }
    println!();

    final_font
}
