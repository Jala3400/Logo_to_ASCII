use std::io;

use clap::Parser;
use imageproc::contrast::threshold;
use logo_to_ascii::{abc, args::Args, proc_image::convert_image};

fn main() -> io::Result<()> {
    let mut args: Args = Args::parse();

    // Load the image
    let mut img = image::open(&args.path).unwrap();

    if args.preprocess {
        let gray_img = img.to_luma8();
        let thresholded_img = threshold(&gray_img, 128);
        img = image::DynamicImage::ImageLuma8(thresholded_img);
    }

    if args.all {
        args.chars = (32..=126).map(|c| c as u8 as char).collect::<String>();
    } else {
        args.chars.push_str(&args.add_chars);
    }

    let font = abc::get_dict8x16(&args.font, &args.chars);

    convert_image(img, args, font);

    Ok(())
}