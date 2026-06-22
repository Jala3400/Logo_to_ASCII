use clap::Parser;
use image::{
    codecs::gif::{GifEncoder, Repeat},
    Frame,
};
use logo_to_ascii::{
    cli::args::Args,
    core::{
        config::ImageConfig,
        errors::L2aError,
        types::{FontBitmap, GifFrame, GifOutput},
    },
    process_gif, process_image,
    processing::gif_ops::composite_gif_frames,
    text::font,
};

fn main() {
    if let Err(e) = run() {
        eprintln!("\n{}\n", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), L2aError> {
    // Parse the command line arguments
    let args = Args::parse();

    // Extract CLI-only fields before consuming args
    let path = args.path.clone();
    let output = args.output.clone();

    // Convert CLI args into the core config and run the pipeline
    let config = ImageConfig::from(args);

    let font_obj = font::load_font(&config)?;
    let font_bitmap = font::build_font_bitmap(&font_obj, &config)?;

    let is_gif = image::ImageFormat::from_path(&path)
        .map(|f| f == image::ImageFormat::Gif)
        .unwrap_or(false);

    if is_gif {
        process_gif_file(&path, output, config, &font_bitmap)?;
    } else {
        process_image_file(&path, output, config, &font_bitmap)?;
    }

    Ok(())
}

fn process_gif_file(
    path: &str,
    output: Option<String>,
    config: ImageConfig,
    font_bitmap: &FontBitmap,
) -> Result<(), L2aError> {
    let file = std::fs::File::open(path)?;
    let raw_frames = composite_gif_frames(file)?;
    let processed = process_gif(raw_frames, &config, font_bitmap)?;

    let font_label = config
        .font_name
        .clone()
        .or_else(|| config.font_path.clone())
        .unwrap_or_else(|| "Ubuntu Mono".to_string());

    let (char_width, char_height) = processed
        .first()
        .map(|f| {
            (
                (f.image.width() as usize + font_bitmap.width - 1) / font_bitmap.width,
                (f.image.height() as usize + font_bitmap.vertical_step - 1)
                    / font_bitmap.vertical_step,
            )
        })
        .unwrap_or((0, 0));

    // Write output GIF if requested — stays sequential, GifEncoder is not Send
    if let Some(ref output_path) = output {
        let out_path = match image::ImageFormat::from_path(output_path) {
            Ok(image::ImageFormat::Gif) => output_path.clone(),
            _ => format!("{}.gif", output_path),
        };
        let mut encoder = GifEncoder::new(std::fs::File::create(&out_path)?);
        encoder.set_repeat(Repeat::Infinite)?;
        for f in &processed {
            encoder.encode_frame(Frame::from_parts(f.image.clone(), 0, 0, f.delay))?;
        }
    }

    let gif_frames: Vec<GifFrame> = processed
        .iter()
        .map(|f| GifFrame {
            ascii: f.ascii.clone(),
            delay_ms: f.delay_ms,
        })
        .collect();

    println!(
        "{}",
        serde_json::to_string_pretty(&GifOutput {
            font: font_label,
            width: char_width,
            height: char_height,
            frames: gif_frames,
        })
        .map_err(|e| L2aError::Other(e.to_string()))?
    );

    Ok(())
}

fn process_image_file(
    path: &str,
    output: Option<String>,
    config: ImageConfig,
    font_bitmap: &FontBitmap,
) -> Result<(), L2aError> {
    // Load the image only when it is not a GIF
    let img = image::open(path)?.to_rgba8();
    let (ascii, processed_img) = process_image(img, config, font_bitmap)?;

    // Print the ASCII art
    print!("{}", ascii);

    // Optionally save the processed image
    if let Some(ref output_path) = output {
        let output_file_path = std::path::Path::new(output_path);

        match image::ImageFormat::from_path(output_file_path) {
            Ok(format) => processed_img.save_with_format(output_path, format),
            Err(_) => processed_img
                .save_with_format(&format!("{}.png", output_path), image::ImageFormat::Png),
        }
        .map_err(L2aError::Image)?
    }

    Ok(())
}
