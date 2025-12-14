use clap::Parser;
use logo_to_ascii::{
    abc,
    args::Args,
    image_ops::{
        add_offset, borders_image, bw_filter, grayscale, negative, resize, saturate,
        treat_transparent,
    },
    proc_image::convert_image,
};
use std::io;

fn main() -> io::Result<()> {
    // Parse the command line arguments
    let mut args: Args = Args::parse();
    args.difference = args.difference % 360;

    // Load the image
    let mut img = image::open(&args.path)
        .unwrap_or_else(|e| panic!("Failed to open image: {}", e))
        .to_rgba8();

    // If the flag indicates it, use all ASCII characters
    if args.all {
        args.chars = (32..=126).map(|c| c as u8 as char).collect::<String>();
    }

    // Add the additional characters
    args.chars.push_str(&args.add_chars);

    // Remove the excluded characters
    args.chars = args
        .chars
        .chars()
        .filter(|c| !args.except.contains(*c))
        .collect();

    if args.chars.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "No characters to use. Please provide valid characters.",
        ));
    }

    // Get the font
    let font = abc::get_dict(&args);

    // Resize the image
    if args.char_width > 0 {
        args.pixel_width = args.char_width * 8;
    }
    if args.char_height > 0 {
        args.pixel_height = args.char_height * 16;
    }
    if args.pixel_height > 0 || args.pixel_width > 0 {
        resize(&mut img, &mut args);
    }

    // Apply the offset
    if args.offsetx != 0 || args.offsety != 0 {
        add_offset(&mut img, &args);
    }

    // Saturate the image
    if args.saturate {
        saturate(&mut img, &args);
    }

    // Add borders
    if args.color_borders || args.border != 0 {
        borders_image(&mut img, &args);
    }

    // Apply the negative effect
    if args.negative {
        negative(&mut img);
    }

    // Always treat transparent pixels, because it makes them visible when printing color
    treat_transparent(&mut img, &args);

    // Grayscale and brighten the image
    if args.grayscale {
        grayscale(&mut img);
    }

    // Apply the black and white filter
    if args.black_and_white {
        bw_filter(&mut img, &args);
    }

    // Convert the image to ASCII
    println!("{}", convert_image(&img, &font, &args));

    // Save the image
    if let Some(output) = &args.output {
        let path = std::path::Path::new(output);

        match image::ImageFormat::from_path(path) {
            Ok(format) => img.save_with_format(output, format),
            Err(_) => img.save_with_format(output.to_owned() + ".png", image::ImageFormat::Png),
        }
        .map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Failed to save image: {}", e))
        })?;
    }

    Ok(())
}
