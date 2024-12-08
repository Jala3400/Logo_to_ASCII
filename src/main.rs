use clap::Parser;
use logo_to_ascii::{
    abc,
    args::Args,
    image_ops::{borders_image, resize},
    proc_image::{black_and_white, convert_bitmap, get_bitmap},
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

    if args.color || args.border != 0 {
        borders_image(&mut img, &args);
    }

    if args.width > 0 {
        args.actual_width = args.width * 8;
    }
    if args.height > 0 {
        args.actual_height = args.height * 16;
    }
    if args.actual_height > 0 || args.actual_width > 0 {
        resize(&mut img, &mut args);
    }

    let bitmap = if args.preprocess {
        black_and_white(&img, &args)
    } else {
        get_bitmap(&img, &args)
    };

    convert_bitmap(&bitmap, &font, &args);

    Ok(())
}
