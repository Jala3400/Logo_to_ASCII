use clap::Parser;
use logo_to_ascii::{
    args::Args, characters, config::ImageConfig, errors::L2aError, font, process_image,
    types::FontBitmap,
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
    let mut config = ImageConfig::from(args);
    characters::process_characters(&mut config);

    let font_obj = font::load_font(&config)?;
    let font_bitmap = font::build_font_bitmap(&font_obj, &config)?;

    let is_gif = image::ImageFormat::from_path(&path)
        .map(|f| f == image::ImageFormat::Gif)
        .unwrap_or(false);

    if is_gif {
        process_gif_file(&path, output, &mut config, &font_bitmap)?;
    } else {
        process_image_file(&path, output, &mut config, &font_bitmap)?;
    }

    Ok(())
}

fn process_gif_file(
    path: &str,
    output: Option<String>,
    config: &mut ImageConfig,
    font_bitmap: &FontBitmap,
) -> Result<(), L2aError> {
    use image::codecs::gif::{GifDecoder, GifEncoder, Repeat};
    use image::AnimationDecoder;
    use image::Frame;

    // Decode frames progressively
    let file = std::fs::File::open(path)?;
    let frames = GifDecoder::new(file)?.into_frames();

    let mut encoder_opt: Option<GifEncoder<std::fs::File>> = if let Some(ref output_path) = output {
        let out_path = match image::ImageFormat::from_path(output_path) {
            Ok(image::ImageFormat::Gif) => output_path.clone(),
            _ => format!("{}.gif", output_path),
        };

        let out_file = std::fs::File::create(&out_path)?;
        let mut encoder = GifEncoder::new(out_file);
        encoder.set_repeat(Repeat::Infinite)?;
        Some(encoder)
    } else {
        None
    };

    // Process each frame progressively
    for frame_result in frames {
        let raw_frame = frame_result?;
        let img = raw_frame.buffer().clone();

        // Process the frame
        let (ascii, processed_img) = process_image(img, config, font_bitmap)?;

        // Print the ASCII art for this frame
        println!("{}", ascii);

        // Setup encoder on first frame and save frames as they're processed
        if let Some(_) = output {
            if let Some(ref mut encoder) = encoder_opt {
                let frame = Frame::from_parts(processed_img, 0, 0, raw_frame.delay());
                encoder.encode_frame(frame)?;
            }
        }
    }

    Ok(())
}

fn process_image_file(
    path: &str,
    output: Option<String>,
    mut config: &mut ImageConfig,
    font_bitmap: &FontBitmap,
) -> Result<(), L2aError> {
    // Load the image only when it is not a GIF
    let img = image::open(path)?.to_rgba8();
    let (ascii, processed_img) = process_image(img, &mut config, font_bitmap)?;

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
