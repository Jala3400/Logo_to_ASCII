use clap::Parser;
use l2a::{
    args::Args,
    characters::process_characters,
    font,
    image_ops::{
        add_padding, borders_image, bw_filter, center_image, grayscale, negative, resize, saturate,
        treat_transparent,
    },
    proc_image::convert_image,
};
use std::io;
use std::num::NonZeroU32;

fn main() -> io::Result<()> {
    // Parse the command line arguments
    let mut args: Args = Args::parse();

    // Load the image
    let mut img = image::open(&args.path)
        .unwrap_or_else(|e| panic!("Failed to open image: {}", e))
        .to_rgba8();

    process_characters(&mut args);

    // Get the font
    let font = font::get_font(&args);

    if font.data.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "No characters available to convert the image.".to_string(),
        ));
    }

    // Resize the image (the first thing to do so everything else is with the right dimensions)
    if let Some(w_nz) = args.width_in_chars {
        args.width_in_pixels = NonZeroU32::new(w_nz.get() * font.width as u32);
    }
    if let Some(h_nz) = args.height_in_chars {
        args.height_in_pixels = NonZeroU32::new(h_nz.get() * font.vertical_step as u32);
    }
    if args.height_in_pixels.is_some() || args.width_in_pixels.is_some() {
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
        borders_image(
            &mut img,
            &args,
            args.border_thickness.map_or(font.width as u32, |t| t.get()),
        );
    }

    // Always treat transparent pixels (before negative so transparent pixels are visible)
    treat_transparent(&mut img, &args);

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
