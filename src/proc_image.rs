use std::collections::HashMap;

use image::{DynamicImage, GenericImageView, ImageBuffer};

use imageproc::{
    contours::find_contours, contrast::threshold, drawing::draw_line_segment_mut, point::Point,
};

use crate::{
    args::Args,
    proc_block::{match_group_with_letter, process_block_brightness, process_block_color},
};

pub fn convert_image(img: DynamicImage, args: Args, font: HashMap<char, Vec<Vec<f32>>>) {
    // Get the dimensions of the image
    let (width, height) = img.dimensions();

    // Calculate number of 8x16 groups
    let num_groups_x = (width + 7) / 8;
    let num_groups_y = (height + 15) / 16;

    println!("Image dimensions: {}x{}", width, height);
    println!("Number of 8x16 groups: {}x{}", num_groups_x, num_groups_y);

    // Iterate over 8x16 groups
    for y in 0..num_groups_y {
        for x in 0..num_groups_x {
            process_block(&img, x, y, &args, &font);
        }
        println!();
    }
}

fn process_block(
    img: &image::DynamicImage,
    x: u32,
    y: u32,
    args: &Args,
    font: &HashMap<char, Vec<Vec<f32>>>,
) {
    let group = if args.color {
        process_block_color(img, x, y, args.inverse, args.num_colors)
    } else {
        process_block_brightness(img, x, y, args.inverse)
    };

    let letter = match_group_with_letter(group, &font);
    print!("{}", letter);
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
