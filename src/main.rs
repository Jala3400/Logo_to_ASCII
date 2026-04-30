use clap::Parser;
use logo_to_ascii::{
    args::Args, characters, config::ImageConfig, errors::L2aError, font, process_image,
    types::{FontBitmap, GifFrame, GifOutput},
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
    use image::codecs::gif::{GifDecoder, GifEncoder, Repeat};
    use image::AnimationDecoder;
    use image::Frame;
    use image::ImageDecoder;

    // Decode frames progressively
    let file = std::fs::File::open(path)?;
    let decoder = GifDecoder::new(file)?;

    // Get canvas dimensions and create a canvas buffer
    let (width, height) = decoder.dimensions();
    let mut canvas = image::RgbaImage::new(width, height);
    let frames = decoder.into_frames();

    // Setup encoder if output is specified
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

    // Determine the font label for the JSON output
    let font_label = config
        .font_name
        .clone()
        .or_else(|| config.font_path.clone())
        .unwrap_or_else(|| "default".to_string());

    let mut gif_frames: Vec<GifFrame> = Vec::new();
    let mut char_width: usize = 0;
    let mut char_height: usize = 0;

    // Process each frame progressively
    for frame_result in frames {
        let raw_frame = frame_result?;

        // Compute delay in milliseconds from the frame's Delay value
        let (numer, denom) = raw_frame.delay().numer_denom_ms();
        let delay_ms = if denom == 0 {
            0u64
        } else {
            (numer as u64) / (denom as u64)
        };

        // Get frame position and dimensions
        let x = raw_frame.left() as u32;
        let y = raw_frame.top() as u32;
        let frame_width = raw_frame.buffer().width();
        let frame_height = raw_frame.buffer().height();

        // Optimize: if frame covers the entire canvas, just use it directly
        if x == 0 && y == 0 && frame_width == width && frame_height == height {
            canvas = raw_frame.buffer().clone();
        } else {
            // Composite the frame onto the canvas at its correct position
            for py in 0..frame_height {
                for px in 0..frame_width {
                    let canvas_x = x + px;
                    let canvas_y = y + py;
                    if canvas_x < width && canvas_y < height {
                        let pixel = *raw_frame.buffer().get_pixel(px, py);
                        canvas.put_pixel(canvas_x, canvas_y, pixel);
                    }
                }
            }
        }

        // Process the full canvas frame
        let (ascii, processed_img) = process_image(canvas.clone(), config.clone(), font_bitmap)?;

        // Derive character dimensions from the processed image (only needed once)
        if char_width == 0 {
            char_width = (processed_img.width() as usize + font_bitmap.width - 1)
                / font_bitmap.width;
            char_height = (processed_img.height() as usize + font_bitmap.vertical_step - 1)
                / font_bitmap.vertical_step;
        }

        gif_frames.push(GifFrame { ascii, delay_ms });

        // Save the full frame to the output GIF if requested
        if let Some(ref mut encoder) = encoder_opt {
            let frame = Frame::from_parts(processed_img, 0, 0, raw_frame.delay());
            encoder.encode_frame(frame)?;
        }
    }

    // Emit the JSON output
    let output_json = GifOutput {
        font: font_label,
        width: char_width,
        height: char_height,
        frames: gif_frames,
    };
    println!("{}", serde_json::to_string_pretty(&output_json)
        .map_err(|e| L2aError::Other(e.to_string()))?);

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
