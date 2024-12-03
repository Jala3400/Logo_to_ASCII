use clap::Parser;
use logo_to_ascii::{
    abc,
    args::Args,
    image_ops::{black_and_white, borders_image, resize},
    proc_image::{convert_bitmap, get_bitmap},
};
use std::io;

fn main() -> io::Result<()> {
    let mut args: Args = Args::parse();
    args.difference = args.difference % 360;

    if args.all {
        args.chars = (32..=126).map(|c| c as u8 as char).collect::<String>();
    } else {
        args.chars.push_str(&args.add_chars);
    }
    args.chars = args
        .chars
        .chars()
        .filter(|c| !args.except.contains(*c))
        .collect();

    let font = abc::get_dict(&args);

    // Load the image
    let mut img = image::open(&args.path).unwrap_or_else(|e| panic!("Failed to open image: {}", e));

    // Resize the image
    match (
        args.actual_height,
        args.actual_width,
        args.height,
        args.width,
    ) {
        (h, w, _, _) if h > 0 || w > 0 => resize(&mut img, &mut args),
        (_, _, h, w) if h > 0 || w > 0 => {
            args.actual_height = w * 8;
            args.actual_width = h * 16;
            resize(&mut img, &mut args);
        }
        _ => (),
    }

    if args.color || args.border != 0 {
        borders_image(&mut img, &args);
    }

    let bitmap = if args.preprocess {
        black_and_white(&img, &args)
    } else {
        get_bitmap(&img, &args)
    };

    convert_bitmap(&bitmap, &font, &args);

    Ok(())
}
