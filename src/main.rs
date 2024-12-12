use clap::Parser;
use image::GenericImageView;
use logo_to_ascii::{
    abc,
    args::Args,
    image_ops::{borders_image, resize},
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

    if args.saturate {
        let mut r_img = img.to_rgba8();
        for pixel in r_img.pixels_mut() {
            let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
            let max = r.max(g).max(b);
            let factor = 255.0 / max as f32;

            if (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) > 127.0 {
                pixel[0] = (r as f32 * factor).round() as u8;
                pixel[1] = (g as f32 * factor).round() as u8;
                pixel[2] = (b as f32 * factor).round() as u8;
            } else {
                pixel[0] = (r as f32 / factor).round() as u8;
                pixel[1] = (g as f32 / factor).round() as u8;
                pixel[2] = (b as f32 / factor).round() as u8;
            }
        }
        img = image::DynamicImage::ImageRgba8(r_img);
    }

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

    if args.offsetx != 0 || args.offsety != 0 {
        let (img_width, img_height) = img.dimensions();
        let new_width = img_width + args.offsetx as u32;
        let new_height = img_height + args.offsety as u32;
        let pixel_bytes = img.color().bytes_per_pixel() as usize;

        let mut new_bytes = vec![0; (new_width * new_height) as usize * pixel_bytes];
        let original_bytes = img.into_bytes();

        // Copy original image data with offset
        for y in 0..img_height {
            let src_start = (y * img_width) as usize * pixel_bytes;
            let src_end = src_start + (img_width as usize * pixel_bytes);
            let dst_start = ((y + args.offsety as u32) * new_width + args.offsetx as u32) as usize
                * pixel_bytes;

            new_bytes[dst_start..dst_start + (img_width as usize * pixel_bytes)]
                .copy_from_slice(&original_bytes[src_start..src_end]);
        }

        img = image::DynamicImage::ImageRgba8(
            image::RgbaImage::from_raw(new_width, new_height, new_bytes)
                .expect("Failed to create offset image"),
        );
    }

    if args.preprocess {
        let img_gray = img.to_luma_alpha8();
        let mut bytes = img_gray.into_raw();
        for chunk in bytes.chunks_mut(2) {
            chunk[0] = if chunk[0] > args.threshold { 255 } else { 0 };
        }
        img = image::DynamicImage::ImageLumaA8(
            image::GrayAlphaImage::from_raw(img.width(), img.height(), bytes)
                .expect("Failed to create black and white image"),
        );
    }

    convert_image(&img, &font, &args);
    Ok(())
}
