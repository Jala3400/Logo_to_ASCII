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
    if args.actual_height > 0 || args.actual_width > 0 {
        let (width, height) = img.dimensions();
        println!("Original dimensions {}x{}", width, height);

        if args.actual_width == 0 {
            let ratio = args.actual_height as f32 / height as f32;
            args.actual_width = (width as f32 * ratio) as u32;
        }
        if args.actual_height == 0 {
            let ratio = args.actual_width as f32 / width as f32;
            args.actual_height = (height as f32 * ratio) as u32;
        }

        img = img.resize_exact(
            args.actual_width,
            args.actual_height,
            image::imageops::FilterType::Lanczos3,
        );
    } else if args.height > 0 || args.width > 0 {
        let (width, height) = img.dimensions();
        println!("Original dimensions {}x{}", width, height);
        args.width = args.width * 8;
        args.height = args.height * 16;

        if args.width == 0 {
            let ratio = args.height as f32 / height as f32;
            args.width = (width as f32 * ratio) as u32;
        }
        if args.height == 0 {
            let ratio = args.width as f32 / width as f32;
            args.height = (height as f32 * ratio) as u32;
        }

        img = img.resize_exact(
            args.width,
            args.height,
            image::imageops::FilterType::Lanczos3,
        );
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
