use crate::{config::ImageConfig, process_image};
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

/// Convert an image to ASCII art.
///
/// # Arguments
///
/// * `image_bytes` — raw bytes of the image file (PNG, JPEG, GIF, …).
///   Pass the result of `new Uint8Array(await file.arrayBuffer())`.
///
/// * `config` — a plain JS object whose keys match the fields of [`ImageConfig`].
///   All fields are optional and fall back to their defaults (same defaults as
///   the CLI). Only provide the fields you want to override. Example:
///
/// ```js
/// import init, { convert } from './l2a.js';
/// await init();
///
/// const bytes = new Uint8Array(await file.arrayBuffer());
/// const result = convert(bytes, {
///     chars: "8dbqp'. ",
///     char_size: 16,
///     print_color: true,
///     format: "html",
/// });
///
/// // Display the ASCII art
/// document.getElementById("ascii").innerHTML = result.ascii;
///
/// // Display the processed image
/// const url = URL.createObjectURL(
///     new Blob([result.image_png], { type: "image/png" })
/// );
/// document.getElementById("preview").src = url;
/// ```
///
/// # Errors
///
/// Returns a JS error string if the image cannot be decoded or the pipeline fails.
#[wasm_bindgen]
pub fn convert(image_bytes: &[u8], config: JsValue) -> Result<ConvertResult, JsValue> {
    let cfg: ImageConfig =
        serde_wasm_bindgen::from_value(config).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let img = image::load_from_memory(image_bytes)
        .map_err(|e| JsValue::from_str(&e.to_string()))?
        .to_rgba8();

    let (ascii, processed_img) =
        process_image(img, cfg).map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Encode the processed image as PNG bytes
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
