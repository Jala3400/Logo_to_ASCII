use crate::{
    config::ImageConfig,
    errors::L2aError,
    proc_pixel::calculate_brightness,
    types::{CharInfo, FontBitmap},
};
#[cfg(not(target_arch = "wasm32"))]
use font_kit::{family_name::FamilyName, properties::Properties, source::SystemSource};
use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use std::vec;

/// The raw bytes of the embedded default font, baked into the binary at compile time.
pub const DEFAULT_FONT_BYTES: &[u8] =
    include_bytes!("../fonts/UbuntuMono/UbuntuMonoNerdFont-Regular.ttf");

/// Returns the embedded default font as a `Font<'static>`.

pub fn default_font() -> Result<Font<'static>, L2aError> {
    Font::try_from_bytes(DEFAULT_FONT_BYTES)
        .ok_or_else(|| L2aError::Font("Built-in font data is invalid".to_owned()))
}

#[cfg(not(target_arch = "wasm32"))]
/// **CLI only.** Loads a font according to the configuration:
/// - `config.font_path` — read from a file path
/// - `config.font_name` — look up via the system font registry
/// - neither — falls back to the embedded [`default_font`]
///
/// Returns a `Font<'static>` so it can outlive the config and be passed freely.
pub fn load_font(config: &ImageConfig) -> Result<Font<'static>, L2aError> {
    if let Some(font_path) = config.font_path.as_ref() {
        let font_data = std::fs::read(font_path).map_err(L2aError::Io)?;

        if config.verbose {
            println!("Loaded font from path: {}", font_path);
        }

        Font::try_from_vec(font_data).ok_or_else(|| L2aError::Font("Invalid font data".to_owned()))
    } else if let Some(font_name) = config.font_name.as_ref() {
        // Use font-kit to look up the font by name
        let source = SystemSource::new();
        let handle = source
            .select_best_match(
                &[FamilyName::Title(font_name.clone())],
                &Properties::default(),
            )
            .map_err(|e| L2aError::Font(format!("Failed to find font: {}", e)))?;

        let font_data = handle
            .load()
            .map_err(|e| L2aError::Font(format!("Failed to load font data: {}", e)))?
            .copy_font_data()
            .ok_or_else(|| L2aError::Font("Failed to copy font data".to_string()))?;

        if config.verbose {
            match handle {
                font_kit::handle::Handle::Path { path, .. } => {
                    println!("Loaded font from path: {}", path.display());
                }
                font_kit::handle::Handle::Memory { .. } => {
                    println!("Loaded font from memory");
                }
            }
        }

        Font::try_from_vec(font_data.to_vec())
            .ok_or_else(|| L2aError::Font("Invalid font data".to_owned()))
    } else {
        default_font()
    }
}

/// Builds a [`FontBitmap`] by rendering each character in `config.chars` using
/// the supplied font.
///
/// This is an I/O-free step
/// The caller is responsible for supplying an appropriate font:
/// - **CLI**: obtain one from [`load_font`]
/// - **WASM**: obtain the embedded font from [`default_font`]
pub fn build_font_bitmap(font: &Font<'_>, config: &ImageConfig) -> Result<FontBitmap, L2aError> {
    let mut img;

    // Define text properties
    let scale = Scale::uniform(config.char_size.get() as f32);
    let color = Rgba([255, 255, 255, 255]);

    // Get font metrics to determine character dimensions
    let v_metrics = font.v_metrics(scale);
    let height = (v_metrics.ascent - v_metrics.descent).ceil() as usize;
    let line_gap = v_metrics.line_gap.ceil() as usize;

    // Use a reference character to get the width (for monospace fonts)
    let glyph = font.glyph('W').scaled(scale);
    let width = glyph.h_metrics().advance_width.ceil() as usize;

    // Create a vector of characters directly from the input string
    let characters: Vec<char> = config.chars.chars().collect();

    let pixel_count = (width * height) as f32;
    let mut final_font = FontBitmap {
        data: Vec::new(),
        width,
        height,
        vertical_step: height + line_gap,
    };

    let mut unsupported_chars = Vec::new();

    // Create a character for each character in the input string
    for i in 0..characters.len() {
        // Create a character for each character in the input string
        let ch = characters[i];
        // id 0 is the missing glyph
        if font.glyph(ch).id().0 == 0 {
            unsupported_chars.push(ch);
            continue;
        }

        // Create an image and then print the character on it
        let mut character: Vec<f32> = vec![0.0; width * height];
        img = RgbaImage::new(width as u32, height as u32);
        let character_string = characters[i].to_string();
        draw_text_mut(&mut img, color, 0, 0, scale, &font, &character_string);

        let mut bright_pixels = 0;
        let mut avg_brightness = 0.0;
        let mut sum = 0.0;
        let mut sum_squares = 0.0;

        // Get the brightness of each pixel from the image and calculate statistics in one pass
        for y in 0..img.height() {
            for x in 0..img.width() {
                let pixel = img.get_pixel(x, y);
                let brightness = calculate_brightness(pixel);
                avg_brightness += brightness;
                let custom_brightness = brightness - 0.5;
                if custom_brightness > 0.0 {
                    bright_pixels += 1;
                }
                character[y as usize * width + x as usize] = custom_brightness;
                sum += custom_brightness;
                sum_squares += custom_brightness * custom_brightness;
            }
        }

        // Calculate statistics from accumulated sums
        let mean = sum / pixel_count;
        let variance = (sum_squares / pixel_count) - (mean * mean);
        let std = variance.sqrt();
        let norm = sum_squares.sqrt();

        // Add the character in the final font
        let char_info = CharInfo {
            char: characters[i],
            data: character,
            min: bright_pixels / 2,
            avg_brightness: avg_brightness / pixel_count,
            norm,
            mean,
            std,
        };
        final_font.insert_ord(char_info);
    }

    // Warn about unsupported characters
    if !unsupported_chars.is_empty() {
        eprintln!(
            "Warning - unsupported characters: {}",
            unsupported_chars.iter().collect::<String>()
        );
    }

    // Print all characters in the final font (ordered)
    if config.verbose {
        print!("Characters: ");
        for char_info in &final_font.data {
            print!("{}", char_info.char);
        }
        println!();
        println!(
            "Char size: {}x{}, Line gap: {}",
            final_font.width, final_font.height, line_gap
        );
        println!(
            "Block size: {}x{}",
            final_font.width, final_font.vertical_step
        );
    }

    Ok(final_font)
}
