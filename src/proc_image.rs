use crate::{
    args::Args,
    proc_block::match_group_with_letter,
    proc_pixel::{calc_custom_brightness, hue_difference},
};
use image::{DynamicImage, GenericImageView, ImageBuffer};
use imageproc::{
    contours::find_contours, contrast::threshold, drawing::draw_line_segment_mut, point::Point,
};
use std::collections::HashMap;

pub fn convert_bitmap(img: Vec<Vec<u8>>, font: HashMap<char, Vec<Vec<u8>>>) {
    // Get the dimensions of the image
    let height = img.len();
    let width = img[0].len();

    // Calculate number of 8x16 groups
    let num_groups_x = (width + 7) / 8;
    let num_groups_y = (height + 15) / 16;

    println!("Image dimensions: {}x{}", width, height);
    println!("Number of 8x16 groups: {}x{}", num_groups_x, num_groups_y);

    // Iterate over 8x16 groups
    for y in 0..num_groups_y {
        for x in 0..num_groups_x {
            let mut group = [[0u8; 8]; 16];
            for by in 0..16 {
                for bx in 0..8 {
                    let iy = y * 16 + by;
                    let ix = x * 8 + bx;
                    if iy < height && ix < width {
                        group[by][bx] = img[iy][ix];
                    }
                }
            }
            print!("{}", match_group_with_letter(group, &font));
        }
        println!();
    }
}

pub fn get_bitmap(img: DynamicImage, args: &Args) -> Vec<Vec<u8>> {
    let mut bitmap = Vec::new();
    for y in 0..img.height() {
        let mut row = Vec::new();
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            row.push(calc_custom_brightness(pixel, args.inverse));
        }
        bitmap.push(row);
    }
    bitmap
}

pub fn preprocess_image(img: DynamicImage, args: &Args) -> DynamicImage {
    let gray_img = img.to_luma8();
    let bw_image = threshold(&gray_img, args.threshold);
    if args.border != 0 {
        let contours = find_contours(&bw_image);

        // Visualize the contours
        let mut output = ImageBuffer::new(img.width(), img.height());

        // Draw ticker borders
        let border_thickness = args.border;
        for contour in contours {
            let points = &contour.points;
            for i in 0..points.len() {
                let p1: Point<i32> = points[i];
                let p2 = points[(i + 1) % points.len()];
                for t in 0..border_thickness {
                    draw_line_segment_mut(
                        &mut output,
                        (
                            (p1.x + t as i32 - border_thickness as i32 / 2) as f32,
                            p1.y as f32,
                        ),
                        (
                            (p2.x + t as i32 - border_thickness as i32 / 2) as f32,
                            p2.y as f32,
                        ),
                        image::Luma([255u8]),
                    );
                    draw_line_segment_mut(
                        &mut output,
                        (
                            p1.x as f32,
                            (p1.y + t as i32 - border_thickness as i32 / 2) as f32,
                        ),
                        (
                            p2.x as f32,
                            (p2.y + t as i32 - border_thickness as i32 / 2) as f32,
                        ),
                        image::Luma([255u8]),
                    );
                }
            }
        }

        DynamicImage::ImageLuma8(output)
    } else {
        DynamicImage::ImageLuma8(bw_image)
    }
}

pub fn preprocess_image_color(img: DynamicImage, args: &Args) -> DynamicImage {
    let mut img = img.to_rgba8();
    let borders = detect_color_borders(&DynamicImage::ImageRgba8(img.clone()), args.threshold);
    for (x, y) in borders {
        img.put_pixel(x, y, image::Rgba([0, 0, 0, 255]));
    }

    let gray_img = image::imageops::grayscale(&img);
    let bw_image = threshold(&gray_img, args.threshold);

    DynamicImage::ImageLuma8(bw_image)
}

//* Colors
fn detect_color_borders(img: &image::DynamicImage, threshold: u8) -> Vec<(u32, u32)> {
    let mut borders = Vec::new();
    let (width, height) = img.dimensions();

    for y in 0..height - 1 {
        for x in 0..width - 1 {
            let current_pixel = img.get_pixel(x, y);
            let right_pixel = img.get_pixel(x + 1, y);
            let bottom_pixel = img.get_pixel(x, y + 1);

            if hue_difference(&current_pixel, &right_pixel) > threshold as f32
                || hue_difference(&current_pixel, &bottom_pixel) > threshold as f32
            {
                borders.push((x, y));
            }
        }
    }

    borders
}
