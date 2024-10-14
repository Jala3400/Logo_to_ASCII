use clap::Parser;
use logo_to_ascii::{
    abc,
    args::Args,
    proc_image::{convert_bitmap, get_bitmap, preprocess_image},
};
use std::io;

fn main() -> io::Result<()> {
    let mut args: Args = Args::parse();

    // Load the image
    let img = image::open(&args.path).unwrap();

    let preprocessed_img = if args.preprocess || args.color || args.border != 0 {
        preprocess_image(img, &args)
    } else {
        img
    };

    let bitmap = get_bitmap(preprocessed_img, &args);

    if args.all {
        args.chars = (32..=126).map(|c| c as u8 as char).collect::<String>();
    } else {
        args.chars.push_str(&args.add_chars);
    }

    args.difference = args.difference % 360;

    let font = abc::get_dict8x16(&args.font, &args.chars);

    convert_bitmap(bitmap, font);

    Ok(())
}
