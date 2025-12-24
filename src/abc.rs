use crate::{
    args::Args,
    types::{CharInfo, FontBitmap},
};
use fontdue::Font;

/// Returns a FontBitmap with the characters and their brightness values
pub fn get_dict(args: &Args) -> FontBitmap {
    // Load a font
    let font: Font;
    if let Some(font_path) = args.font.as_ref() {
        let font_data =
            std::fs::read(&font_path).expect(&format!("Failed to read font file {font_path}"));
        font = Font::from_bytes(font_data, fontdue::FontSettings::default()).unwrap();
    } else {
        font = Font::from_bytes(
            include_bytes!("../fonts/UbuntuMono-Regular.ttf") as &[u8],
            fontdue::FontSettings::default(),
        )
        .unwrap();
    }

    // Get font metrics using a reference character to determine cell dimensions
    // Use 'W' as it's typically a full-width character
    let font_size = 16.0f32; // Base font size for rasterization
    let line_metrics = font.horizontal_line_metrics(font_size).unwrap();
    let height = (line_metrics.new_line_size).ceil() as usize;

    // Get the advance width from a reference character for consistent width
    let ref_metrics = font.metrics('W', font_size);
    let width = ref_metrics.advance_width.ceil() as usize;

    let cell_size = width * height;

    // Create a vector of characters directly from the input string
    let characters: Vec<char> = args.chars.chars().collect();

    let mut final_font = FontBitmap {
        data: Vec::new(),
        width,
        height,
    };

    // Create a character for each character in the input string
    for i in 0..characters.len() {
        // Rasterize the character using fontdue
        let (metrics, bitmap) = font.rasterize(characters[i], font_size);

        let mut character = vec![-args.midpoint_brightness; cell_size];
        let mut bright_pixels = 0;
        let mut avg_brightness = 0.0;
        let mut sum = 0.0;
        let mut sum_squares = 0.0;

        // Calculate the offset to properly position the glyph within the cell
        // xmin is the left-side bearing (horizontal offset from the origin)
        // ymin is the vertical offset from the baseline
        let x_offset = metrics.xmin.max(0) as usize;
        let y_offset = (line_metrics.ascent - metrics.height as f32 - metrics.ymin as f32)
            .round()
            .max(0.0) as usize;

        // Copy the bitmap into the character array at the correct position
        for gy in 0..metrics.height {
            for gx in 0..metrics.width {
                let cell_x = x_offset + gx;
                let cell_y = y_offset + gy;

                // Only write if within cell bounds
                if cell_x < width && cell_y < height {
                    let glyph_idx = gy * metrics.width + gx;
                    let cell_idx = cell_y * width + cell_x;
                    let brightness = (bitmap[glyph_idx] as f32 / 255.0).sqrt();
                    avg_brightness += brightness;
                    let custom_brightness = brightness - args.midpoint_brightness;
                    if custom_brightness > 0.0 {
                        bright_pixels += 1;
                    }
                    character[cell_idx] = custom_brightness;
                    sum += custom_brightness;
                    sum_squares += custom_brightness * custom_brightness;
                }
            }
        }

        // Calculate statistics from accumulated sums
        let cell_size_f = cell_size as f32;
        let mean = sum / cell_size_f;
        let variance = (sum_squares / cell_size_f) - (mean * mean);
        let std = variance.sqrt();
        let norm = sum_squares.sqrt();

        // Add the character in the final font
        let char_info = CharInfo {
            char: characters[i],
            data: character,
            min: bright_pixels / 2,
            avg_brightness: avg_brightness / cell_size_f,
            norm,
            mean,
            std,
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
