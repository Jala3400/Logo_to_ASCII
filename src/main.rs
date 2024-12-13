use clap::Parser;
use logo_to_ascii::{
    abc,
    args::Args,
    image_ops::{add_offset, borders_image, preprocess, resize, saturate, treat_transparent},
    proc_image::convert_image,
};
use std::io;

fn main() -> io::Result<()> {
    let mut args: Args = Args::parse();
    args.difference = args.difference % 360;

    // Load the image
    let mut img = image::open(&args.path).unwrap_or_else(|e| panic!("Failed to open image: {}", e));

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

    if args.width > 0 {
        args.actual_width = args.width * 8;
    }
    if args.height > 0 {
        args.actual_height = args.height * 16;
    }
    if args.actual_height > 0 || args.actual_width > 0 {
        resize(&mut img, &mut args);
    }

    if args.offsetx != 0 || args.offsety != 0 {
        add_offset(&mut img, &args);
    }

    if args.saturate {
        saturate(&mut img, &args);
    }

    if args.color || args.border != 0 {
        borders_image(&mut img, &args);
    }

    if args.preprocess {
        preprocess(&mut img, &args);
    }

    treat_transparent(&mut img);

    // if args.inverse {
    //     inverse(&mut img);
    // }

    convert_image(&img, &font, &args);
    Ok(())
}
