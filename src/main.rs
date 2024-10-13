use std::io;

use clap::Parser;
use image::ImageBuffer;
use imageproc::{
    contours::find_contours, contrast::threshold, drawing::draw_line_segment_mut, point::Point,
};
use logo_to_ascii::{abc, args::Args, proc_image::convert_image};

fn main() -> io::Result<()> {
    let mut args: Args = Args::parse();

    // Load the image
    let mut img = image::open(&args.path).unwrap();

    if args.preprocess || args.border != 0 {
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

            img = image::DynamicImage::ImageLuma8(output);
        } else {
            img = image::DynamicImage::ImageLuma8(bw_image);
        }
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