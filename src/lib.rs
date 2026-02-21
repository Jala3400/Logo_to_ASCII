#[cfg(not(target_arch = "wasm32"))]
pub mod args;
pub mod characters;
pub mod config;
pub mod errors;
pub mod font;
pub mod image_ops;
pub mod proc_block;
pub mod proc_image;
pub mod proc_pixel;
pub mod types;
#[cfg(target_arch = "wasm32")]
pub mod wasm;

use image::RgbaImage;
use std::num::NonZeroU32;

/// Run the full image-to-ASCII pipeline.
///
/// Accepts a decoded RGBA image and a configuration struct, applies all
/// image processing steps in order, and returns the ASCII string together
/// with the fully-processed image (so CLI callers can optionally save it).
pub fn process_image(
    mut img: RgbaImage,
    mut cfg: config::ImageConfig,
) -> Result<(String, RgbaImage), errors::L2aError> {
    use image_ops::{
        add_padding, borders_image, bw_filter, center_image, grayscale, negative, resize, saturate,
        treat_transparent,
    };

    characters::process_characters(&mut cfg);

    #[cfg(not(target_arch = "wasm32"))]
    let font_obj = font::load_font(&cfg)?;
    #[cfg(target_arch = "wasm32")]
    let font_obj = font::default_font()?;
    let font = font::build_font_bitmap(&font_obj, &cfg)?;

    if font.data.is_empty() {
        return Err(errors::L2aError::NoCharacters);
    }

    // Resolve character-based dimensions to pixels now that we know the font metrics.
    if let Some(w_nz) = cfg.width_in_chars {
        cfg.width_in_pixels = NonZeroU32::new(w_nz.get() * font.width as u32);
    }
    if let Some(h_nz) = cfg.height_in_chars {
        cfg.height_in_pixels = NonZeroU32::new(h_nz.get() * font.vertical_step as u32);
    }

    // Resize the image (the first thing to do so everything else is with the right dimensions)
    if cfg.height_in_pixels.is_some() || cfg.width_in_pixels.is_some() {
        resize(&mut img, &mut cfg);
    }

    // Adjust padding to center the image (after resizing, so it is centered with the final size)
    if cfg.center {
        center_image(&img, &mut cfg, &font);
    }

    // Apply the padding (after resizing) (before borders so borders are included in the padding)
    // (also before saturate and negative so the padding is affected by them)
    if cfg.padding != 0 || cfg.padding_x != 0 || cfg.padding_y != 0 {
        add_padding(&mut img, &cfg)?;
    }

    // Saturate the image (before borders so borders are more visible and before negative so it is not inverted)
    if cfg.saturate {
        saturate(&mut img, &cfg);
    }

    // Add borders (before negative effect so borders are visible)
    if cfg.border_criteria.is_some() {
        borders_image(
            &mut img,
            &cfg,
            cfg.border_thickness.map_or(font.width as u32, |t| t.get()),
        );
    }

    // Always remove transparency using a background color (before negative so the color is inverted if needed)
    treat_transparent(&mut img, &cfg);

    // Apply the negative effect
    if cfg.negative {
        negative(&mut img);
    }

    // Grayscale and brighten the image (after saturate, negative and transparent so it is applied to the final colors)
    if cfg.grayscale {
        grayscale(&mut img);
    }

    // Apply the black and white filter (after saturate, negative and transparent so it is applied to the final colors)
    if cfg.black_and_white {
        bw_filter(&mut img, &cfg);
    }

    let ascii = proc_image::convert_image(&img, &font, &cfg);
    Ok((ascii, img))
}
