use crate::args::Args;
use image::Rgba;

// Calculate the brightness and subtract the midpoint brightness 
pub fn calc_custom_brightness(pixel: &Rgba<u8>, args: &Args) -> f32 {
    calculate_brightness(&pixel) - args.midpoint_brightness
}

// Calculate the brightness of a pixel
pub fn calculate_brightness(pixel: &Rgba<u8>) -> f32 {
    let r = pixel[0] as f32;
    let g = pixel[1] as f32;
    let b = pixel[2] as f32;

    ((0.299 * r + 0.587 * g + 0.114 * b) / 255.0).sqrt()
}

// Calculate the hue of a pixel. If the pixel is transparent, return 720
pub fn calc_custom_hue(pixel: &Rgba<u8>) -> u16 {
    if pixel[3] == 0 {
        // If the pixel is transparent
        720
    } else {
        calc_hue(pixel)
    }
}

// Calculate the hue of a pixel
pub fn calc_hue(pixel: &Rgba<u8>) -> u16 {
    let r = pixel[0] as f32 / 255.0;
    let g = pixel[1] as f32 / 255.0;
    let b = pixel[2] as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);

    let hue = if max == min {
        0.0
    } else {
        let delta = max - min;
        let hue = if max == r {
            (g - b) / delta + if g < b { 6.0 } else { 0.0 }
        } else if max == g {
            (b - r) / delta + 2.0
        } else {
            (r - g) / delta + 4.0
        };
        hue * 60.0
    };

    hue.round() as u16
}

// pub fn calc_hue(pixel: Rgba<u8>) -> u16 {
//     /*Hue = arctan ⁡ 2 ( 3 ⋅ ( G − B) , 2 R − G − B) Hue=arctan2( sqrt(3) ​ ⋅(G−B),2R−G−B) */
//     let r = pixel[0] as f32 / 255.0;
//     let g = pixel[1] as f32 / 255.0;
//     let b = pixel[2] as f32 / 255.0;

//     let hue = (3.0_f32.sqrt() * (g - b)).atan2(2.0 * r - g - b).to_degrees();
//     if hue < 0.0 {
//         (hue + 360.0).round() as u16
//     } else {
//         hue.round() as u16
//     }
// }
//

// Calculate the difference between the brightness of two pixels
pub fn brightness_difference(pixel1: &Rgba<u8>, pixel2: &Rgba<u8>) -> f32 {
    (calculate_brightness(pixel1) - calculate_brightness(pixel2)).abs()
}

// Calculate the difference between the hue of two pixels
pub fn hue_difference(pixel1: &Rgba<u8>, pixel2: &Rgba<u8>) -> u16 {
    let diff = calc_custom_hue(pixel1).abs_diff(calc_custom_hue(pixel2));
    diff.min(360_u16.abs_diff(diff))
}
