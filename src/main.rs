use clap::Parser;
use logo_to_ascii::{
    args::Args, characters, config::ImageConfig, errors::L2aError, font, process_gif,
    process_image, types::FontBitmap,
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

    // Decode all frames upfront so we can access both pixel data and timing metadata
    let file = std::fs::File::open(path)?;
    let raw_frames = GifDecoder::new(file)?.into_frames().collect_frames()?;

    let imgs: Vec<image::RgbaImage> = raw_frames.iter().map(|f| f.buffer().clone()).collect();
    let processed = process_gif(imgs, config, font_bitmap)?;

    // Print the ASCII art for every frame
    for (ascii, _) in &processed {
        println!("{}", ascii);
    }

    // Optionally save as an animated GIF, preserving the original frame delays
    if let Some(ref output_path) = output {
        let out_path = match image::ImageFormat::from_path(output_path) {
            Ok(image::ImageFormat::Gif) => output_path.clone(),
            _ => format!("{}.gif", output_path),
        };

        let out_file = std::fs::File::create(&out_path)?;
        let mut encoder = GifEncoder::new(out_file);
        encoder.set_repeat(Repeat::Infinite)?;

        for ((_, processed_img), raw_frame) in processed.into_iter().zip(raw_frames.iter()) {
            let frame = Frame::from_parts(processed_img, 0, 0, raw_frame.delay());
            encoder.encode_frame(frame)?;
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
