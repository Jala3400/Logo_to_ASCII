use image::Rgba;

pub fn calc_custom_brightness(pixel: Rgba<u8>, inverse: bool) -> f32 {
    if pixel[3] == 0 {
        // If the pixel is transparent
        -0.5
    } else {
        (calculate_brightness(&pixel) - 0.5) * if inverse { -1.0 } else { 1.0 }
    }
}

pub fn calculate_brightness(pixel: &Rgba<u8>) -> f32 {
    let r = pixel[0] as f32 / 255.0;
    let g = pixel[1] as f32 / 255.0;
    let b = pixel[2] as f32 / 255.0;

    let brightness = (0.299 * r + 0.587 * g + 0.114 * b).sqrt();
    brightness
}

pub fn calc_hue(pixel: Rgba<u8>) -> u8 {
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

    hue.round() as u8
}

// fn calculate_hue(pixel: Rgba<u8>) -> u8 {
//     /*Hue = arctan ⁡ 2 ( 3 ⋅ ( G − B) , 2 R − G − B) Hue=arctan2( sqrt(3) ​ ⋅(G−B),2R−G−B) */
//     let r = pixel[0] as f32 / 255.0;
//     let g = pixel[1] as f32 / 255.0;
//     let b = pixel[2] as f32 / 255.0;

//     let hue = (3.0_f32.sqrt() * (g - b)).atan2(2.0 * r - g - b).to_degrees();
//     if hue < 0.0 {
//         (hue + 360.0).round() as u8
//     } else {
//         hue.round() as u8
//     }
// }