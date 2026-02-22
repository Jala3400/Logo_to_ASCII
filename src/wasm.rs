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

/// Convert an image using the embedded default font.
#[wasm_bindgen]
pub fn convert(image_bytes: &[u8], config: JsValue) -> Result<ConvertResult, JsValue> {
    let cfg: ImageConfig =
        serde_wasm_bindgen::from_value(config).map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Use the embedded default font (Ubuntu Mono)
    let font_obj = crate::font::default_font().map_err(|e| JsValue::from_str(&e.to_string()))?;
    let font = crate::font::build_font_bitmap(&font_obj, &cfg)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

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

/// Runs `process_characters` on `config` and returns the resulting character set.
#[wasm_bindgen]
pub fn get_final_chars(config: JsValue) -> Result<String, JsValue> {
    let mut cfg: ImageConfig =
        serde_wasm_bindgen::from_value(config).map_err(|e| JsValue::from_str(&e.to_string()))?;
    crate::characters::process_characters(&mut cfg);
    Ok(cfg.chars)
}
