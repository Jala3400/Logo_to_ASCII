use crate::{
    characters,
    config::ImageConfig,
    font, process_image,
    types::{CharInfo, FontBitmap},
};
use wasm_bindgen::prelude::*;

/// The result of a [`convert`] call, containing both outputs of the pipeline.
#[wasm_bindgen]
pub struct ConvertResult {
    ascii: String,
    image_png: Vec<u8>,
}

#[wasm_bindgen]
impl ConvertResult {
    /// The ASCII art string.
    /// For colored output with `format: "html"` this is an HTML fragment
    /// wrapped in a `<pre>` tag, ready to set as `innerHTML`.
    /// For plain or ANSI output use it as `textContent` / `innerText`.
    #[wasm_bindgen(getter)]
    pub fn ascii(&self) -> String {
        self.ascii.clone()
    }

    /// The fully-processed image as raw PNG bytes.
    ///
    /// To display it in the browser:
    /// ```js
    /// const url = URL.createObjectURL(
    ///     new Blob([result.image_png], { type: "image/png" })
    /// );
    /// document.getElementById("preview").src = url;
    /// ```
    #[wasm_bindgen(getter)]
    pub fn image_png(&self) -> Vec<u8> {
        self.image_png.clone()
    }
}

/// Convert an image using a font bitmap ripped from the browser canvas.
///
/// `font_data` must be a JS object with:
/// - `chars`         — the characters in order (string)
/// - `cell_pixels`   — flat JS `Array` of brightness floats (0.0–1.0), `width × height` per char
/// - `width`         — cell width in pixels (number)
/// - `height`        — cell height in pixels (number)
/// - `vertical_step` — line height in pixels (number)
#[wasm_bindgen]
pub fn convert(
    image_bytes: &[u8],
    font_data: JsValue,
    config: JsValue,
) -> Result<ConvertResult, JsValue> {
    let font_js: JsFontData =
        serde_wasm_bindgen::from_value(font_data).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let font = build_bitmap_from_js(font_js).map_err(|e| JsValue::from_str(&e))?;

    let cfg: ImageConfig =
        serde_wasm_bindgen::from_value(config).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let img = image::load_from_memory(image_bytes)
        .map_err(|e| JsValue::from_str(&e.to_string()))?
        .to_rgba8();

    let (ascii, processed_img) =
        process_image(img, cfg, font).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let mut png_bytes: Vec<u8> = Vec::new();
    processed_img
        .write_to(
            &mut std::io::Cursor::new(&mut png_bytes),
            image::ImageFormat::Png,
        )
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(ConvertResult {
        ascii,
        image_png: png_bytes,
    })
}

/// JS-side font payload: pixel brightness values ripped from a browser canvas.
#[derive(serde::Deserialize)]
struct JsFontData {
    /// The exact characters for which pixel data is provided (same order as cell_pixels).
    chars: String,
    /// Flat brightness array (0.0–1.0), `width × height` values per character.
    cell_pixels: Vec<f32>,
    width: usize,
    height: usize,
    vertical_step: usize,
}

/// Build a [`FontBitmap`] from brightness values ripped from a browser canvas.
fn build_bitmap_from_js(data: JsFontData) -> Result<FontBitmap, String> {
    let chars: Vec<char> = data.chars.chars().collect();
    let cell_size = data.width * data.height;

    if data.cell_pixels.len() != chars.len() * cell_size {
        return Err(format!(
            "Font data mismatch: expected {} pixels ({} chars × {}), got {}",
            chars.len() * cell_size,
            chars.len(),
            cell_size,
            data.cell_pixels.len()
        ));
    }

    let pixel_count = cell_size as f32;
    let mut bitmap = FontBitmap {
        data: Vec::new(),
        width: data.width,
        height: data.height,
        vertical_step: data.vertical_step,
    };

    for (i, &ch) in chars.iter().enumerate() {
        let pixels = &data.cell_pixels[i * cell_size..(i + 1) * cell_size];
        let mut char_data = Vec::with_capacity(cell_size);
        let mut bright_pixels = 0usize;
        let mut avg_brightness = 0.0f32;
        let mut sum = 0.0f32;
        let mut sum_squares = 0.0f32;

        for &brightness in pixels {
            // Match Rust's `custom_brightness = brightness - midpoint` convention
            let custom = brightness - 0.5;
            char_data.push(custom);
            avg_brightness += brightness;
            if custom > 0.0 {
                bright_pixels += 1;
            }
            sum += custom;
            sum_squares += custom * custom;
        }

        let mean = sum / pixel_count;
        let variance = (sum_squares / pixel_count) - (mean * mean);
        let std = variance.sqrt();
        let norm = sum_squares.sqrt();

        bitmap.insert_ord(CharInfo {
            char: ch,
            data: char_data,
            min: bright_pixels / 2,
            avg_brightness: avg_brightness / pixel_count,
            norm,
            mean,
            std,
        });
    }

    Ok(bitmap)
}

/// Runs `process_characters` on `config` and returns the resulting character set.
///
/// Call this before ripping a font so you know exactly which characters
/// (after dict expansion, exclusions, and deduplication) need to be rendered.
#[wasm_bindgen]
pub fn get_final_chars(config: JsValue) -> Result<String, JsValue> {
    let mut cfg: ImageConfig =
        serde_wasm_bindgen::from_value(config).map_err(|e| JsValue::from_str(&e.to_string()))?;
    crate::characters::process_characters(&mut cfg);
    Ok(cfg.chars)
}
