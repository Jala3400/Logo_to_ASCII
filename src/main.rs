use clap::Parser;
use logo_to_ascii::{
    abc,
    args::Args,
    proc_image::{
        borders_image, borders_image_color, convert_bitmap, get_bitmap, to_black_and_white,
    },
};
use std::io;

fn main() -> io::Result<()> {
    let mut args: Args = Args::parse();

    // Load the image
    let img = image::open(&args.path).unwrap();

    let preprocessed_img = match (args.color, args.preprocess, args.border != 0) {
        (true, true, _) => to_black_and_white(borders_image_color(img, &args), &args),
        (true, false, _) => borders_image_color(img, &args),
        (false, true, true) => borders_image(to_black_and_white(img, &args), &args),
        (false, true, false) => to_black_and_white(img, &args),
        (false, false, true) => borders_image(img, &args),
        (false, false, false) => img,
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
