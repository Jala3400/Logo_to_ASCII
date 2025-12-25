use clap::Parser;
use logo_to_ascii::{
    abc,
    args::Args,
    image_ops::{
        add_padding, borders_image, bw_filter, center_image, grayscale, negative, resize, saturate,
        treat_transparent,
    },
    proc_image::convert_image,
};
use std::io;

fn main() -> io::Result<()> {
    // Parse the command line arguments
    let mut args: Args = Args::parse();

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

    // Always treat transparent pixels
    treat_transparent(&mut img, &args);

    // Resize the image (after transparent treatment because of artifacts)
    if args.width_in_chars > 0 {
        args.width_in_pixels = args.width_in_chars * 8;
    }
    if args.height_in_chars > 0 {
        args.height_in_pixels = args.height_in_chars * 16;
    }
    if args.height_in_pixels > 0 || args.width_in_pixels > 0 {
        resize(&mut img, &mut args);
    }

    // Adjust padding to center the image (after resizing, so it is centered with the final size)
    if args.center {
        center_image(&img, &mut args, &font);
    }

    // Apply the padding (after resizing) (before borders so borders are included in the padding)
    // (also before saturate and negative so the padding is affected by them)
    if args.padding_x != 0 || args.padding_y != 0 {
        add_padding(&mut img, &args);
    }

    // Saturate the image (before borders so borders are more visible and before negative so it is not inverted)
    if args.saturate {
        saturate(&mut img, &args);
    }

    // Add borders (before negative effect so borders are visible)
    if args.border_criteria.is_some() {
        borders_image(&mut img, &args);
    }

    // Apply the negative effect
    if args.negative {
        negative(&mut img);
    }

    // Grayscale and brighten the image (after saturate, negative and transparent so it is applied to the final colors)
    if args.grayscale {
        grayscale(&mut img);
    }

    // Apply the black and white filter (after saturate, negative and transparent so it is applied to the final colors)
    if args.black_and_white {
        bw_filter(&mut img, &args);
    }
    // Convert the image to ASCII
    print!("{}", convert_image(&img, &font, &args));

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
