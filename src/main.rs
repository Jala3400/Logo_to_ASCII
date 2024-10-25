use clap::Parser;
use logo_to_ascii::{
    abc,
    args::Args,
    proc_image::{borders_image, convert_bitmap, get_bitmap, to_black_and_white},
};
use std::io;

fn main() -> io::Result<()> {
    let mut args: Args = Args::parse();
    args.difference = args.difference % 360;

    // Load the image
    let mut img = image::open(&args.path).unwrap();

    if args.color || args.border != 0 {
        borders_image(&mut img, &args);
    }
    if args.preprocess {
        img = to_black_and_white(&img, &args);
    }

    let bitmap = get_bitmap(&img, &args);

    if args.all {
        args.chars = (32..=126).map(|c| c as u8 as char).collect::<String>();
    } else {
        args.chars.push_str(&args.add_chars);
    }

    let font = abc::get_dict8x16(&args.font, &args.chars);

    convert_bitmap(&bitmap, &font);

    Ok(())
}
