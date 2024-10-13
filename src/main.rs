use clap::Parser;
use logo_to_ascii::{
    abc,
    args::Args,
    proc_image::{convert_bitmap, get_bitmap, preprocess_image, preprocess_image_color},
};
use std::io;

fn main() -> io::Result<()> {
    let mut args: Args = Args::parse();
    println!("{:?}", u8::MAX);

    // Load the image
    let mut img = image::open(&args.path).unwrap();

    if args.color {
        img = preprocess_image_color(img, &args);
    } else if args.preprocess {
        img = preprocess_image(img, &args);
    }

    let bitmap = get_bitmap(img, &args);

    if args.all {
        args.chars = (32..=126).map(|c| c as u8 as char).collect::<String>();
    } else {
        args.chars.push_str(&args.add_chars);
    }

    let font = abc::get_dict8x16(&args.font, &args.chars);

    convert_bitmap(bitmap, font);

    Ok(())
}
