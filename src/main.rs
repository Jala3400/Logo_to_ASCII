use clap::Parser;
use image::GenericImageView;
use logo_to_ascii::{
    abc,
    args::Args,
    proc_image::{black_and_white, borders_image, convert_bitmap, get_bitmap},
};
use std::io;

fn main() -> io::Result<()> {
    let mut args: Args = Args::parse();
    args.difference = args.difference % 360;

    // Load the image
    let mut img = image::open(&args.path).unwrap_or_else(|e| panic!("Failed to open image: {}", e));

    if args.height > 0 || args.width > 0 {
        let actual_width = args.width * 8;
        let actual_height = args.height * 16;
        let (width, height) = img.dimensions();
        if actual_width > 0 && actual_height > 0 {
            img = img.resize_exact(actual_width, actual_height, image::imageops::FilterType::Lanczos3);
        } else if actual_width > 0 {
            let ratio = actual_width as f32 / width as f32;
            let new_height = (height as f32 * ratio) as u32;
            img = img.resize(actual_width, new_height, image::imageops::FilterType::Lanczos3);
        } else {
            let ratio = actual_height as f32 / height as f32;
            let new_width = (width as f32 * ratio) as u32;
            img = img.resize(new_width, actual_height, image::imageops::FilterType::Lanczos3);
        }
    }

    if args.color || args.border != 0 {
        borders_image(&mut img, &args);
    }

    let bitmap = if args.preprocess {
        black_and_white(&img, &args)
    } else {
        get_bitmap(&img, &args)
    };

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

    convert_bitmap(&bitmap, &font, &args);

    Ok(())
}
