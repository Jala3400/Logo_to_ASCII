use crate::types::{GifFrame, GifOutput};
use crate::{config::ImageConfig, process_image};
use image::codecs::gif::GifDecoder;
use image::AnimationDecoder;
use image::ImageDecoder;
use wasm_bindgen::prelude::*;

// * Image-related structures

/// The result of a [`convert_image`] call, containing both outputs of the pipeline.
#[wasm_bindgen]
pub struct ConvertImageResult {
    ascii: String,
    image_png: Vec<u8>,
}

#[wasm_bindgen]
impl ConvertImageResult {
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
pub fn convert_image(image_bytes: &[u8], config: JsValue) -> Result<ConvertImageResult, JsValue> {
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
        process_image(img, cfg, &font).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let mut png_bytes: Vec<u8> = Vec::new();
    processed_img
        .write_to(
            &mut std::io::Cursor::new(&mut png_bytes),
            image::ImageFormat::Png,
        )
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(ConvertImageResult {
        ascii,
        image_png: png_bytes,
    })
}

/// Runs `process_characters` on `config` and returns the resulting character set.
#[wasm_bindgen]
pub fn get_final_chars(config: JsValue) -> Result<String, JsValue> {
    let cfg: ImageConfig =
        serde_wasm_bindgen::from_value(config).map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(cfg.get_processed_chars())
}

// * GIF-related structures and functions

/// A single GIF frame: raw PNG bytes and the inter-frame delay.
#[wasm_bindgen]
pub struct GifFrameInfo {
    png_bytes: Vec<u8>,
    delay_ms: u64,
}

#[wasm_bindgen]
impl GifFrameInfo {
    /// Raw PNG bytes for this frame.
    ///
    /// ```js
    /// const url = URL.createObjectURL(
    ///     new Blob([frame.png_bytes], { type: "image/png" })
    /// );
    /// ```
    #[wasm_bindgen(getter)]
    pub fn png_bytes(&self) -> Vec<u8> {
        self.png_bytes.clone()
    }

    /// How long this frame should be displayed, in milliseconds.
    #[wasm_bindgen(getter)]
    pub fn delay_ms(&self) -> u64 {
        self.delay_ms
    }
}

/// The result of a [`convert_gif`] call.
#[wasm_bindgen]
pub struct ConvertGifResult {
    /// Serialised [`GifOutput`] JSON (font, width, height, frames[]{ascii, delay_ms}).
    ascii_json: String,
    original_gif: Vec<GifFrameInfo>,
    processed_gif: Vec<GifFrameInfo>,
}

#[wasm_bindgen]
impl ConvertGifResult {
    /// The ASCII/JSON output as a pretty-printed JSON string.
    ///
    /// ```js
    /// const data = JSON.parse(result.ascii_json);
    /// // data.frames[i].ascii  — ASCII art for frame i
    /// // data.frames[i].delay_ms
    /// ```
    #[wasm_bindgen(getter)]
    pub fn ascii_json(&self) -> String {
        self.ascii_json.clone()
    }

    /// Original (un-processed) frames as a JS Array of [`GifFrameInfo`].
    ///
    /// ```js
    /// for (const frame of result.original_gif) {
    ///     const url = URL.createObjectURL(
    ///         new Blob([frame.png_bytes], { type: "image/png" })
    ///     );
    /// }
    /// ```
    #[wasm_bindgen(getter)]
    pub fn original_gif(&self) -> Vec<GifFrameInfo> {
        self.original_gif
            .iter()
            .map(|f| GifFrameInfo {
                png_bytes: f.png_bytes.clone(),
                delay_ms: f.delay_ms,
            })
            .collect()
    }

    /// ASCII-processed frames as a JS Array of [`GifFrameInfo`].
    #[wasm_bindgen(getter)]
    pub fn processed_gif(&self) -> Vec<GifFrameInfo> {
        self.processed_gif
            .iter()
            .map(|f| GifFrameInfo {
                png_bytes: f.png_bytes.clone(),
                delay_ms: f.delay_ms,
            })
            .collect()
    }
}

/// Convert a GIF using the embedded default font.
///
/// `gif_bytes` — the raw bytes of a `.gif` file (e.g. from `File.arrayBuffer()`).
/// `config`    — an [`ImageConfig`]-shaped JS object (same schema as `convert_image`).
#[wasm_bindgen]
pub fn convert_gif(gif_bytes: &[u8], config: JsValue) -> Result<ConvertGifResult, JsValue> {
    let cfg: ImageConfig =
        serde_wasm_bindgen::from_value(config).map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Build font bitmap (same as convert_image)
    let font_obj = crate::font::default_font().map_err(|e| JsValue::from_str(&e.to_string()))?;
    let font = crate::font::build_font_bitmap(&font_obj, &cfg)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Decode GIF from memory
    let cursor = std::io::Cursor::new(gif_bytes);
    let decoder = GifDecoder::new(cursor).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let (width, height) = decoder.dimensions();
    let mut canvas = image::RgbaImage::new(width, height);
    let frames = decoder.into_frames();

    // Determine the font label for JSON output
    let font_label = cfg
        .font_name
        .clone()
        .or_else(|| cfg.font_path.clone())
        .unwrap_or_else(|| "default".to_string());

    let mut gif_frames: Vec<GifFrame> = Vec::new();
    let mut original_frames: Vec<GifFrameInfo> = Vec::new();
    let mut processed_frames: Vec<GifFrameInfo> = Vec::new();
    let mut char_width: usize = 0;
    let mut char_height: usize = 0;

    for frame_result in frames {
        let raw_frame = frame_result.map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Delay in milliseconds
        let (numer, denom) = raw_frame.delay().numer_denom_ms();
        let delay_ms = if denom == 0 {
            0u64
        } else {
            (numer as u64) / (denom as u64)
        };

        // Composite frame onto canvas
        let x = raw_frame.left() as u32;
        let y = raw_frame.top() as u32;
        let frame_width = raw_frame.buffer().width();
        let frame_height = raw_frame.buffer().height();

        if x == 0 && y == 0 && frame_width == width && frame_height == height {
            canvas = raw_frame.buffer().clone();
        } else {
            for py in 0..frame_height {
                for px in 0..frame_width {
                    let cx = x + px;
                    let cy = y + py;
                    if cx < width && cy < height {
                        canvas.put_pixel(cx, cy, *raw_frame.buffer().get_pixel(px, py));
                    }
                }
            }
        }

        // Encode the original (composited) canvas as PNG
        let original_png =
            rgba_image_to_png_bytes(&canvas).map_err(|e| JsValue::from_str(&e.to_string()))?;

        original_frames.push(GifFrameInfo {
            png_bytes: original_png,
            delay_ms,
        });

        // Run the ASCII pipeline on the canvas
        let (ascii, processed_img) = process_image(canvas.clone(), cfg.clone(), &font)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Derive character grid dimensions once
        if char_width == 0 {
            char_width = (processed_img.width() as usize + font.width - 1) / font.width;
            char_height =
                (processed_img.height() as usize + font.vertical_step - 1) / font.vertical_step;
        }

        // Encode the processed frame as PNG
        let processed_png = rgba_image_to_png_bytes(&processed_img)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        processed_frames.push(GifFrameInfo {
            png_bytes: processed_png,
            delay_ms,
        });

        gif_frames.push(GifFrame { ascii, delay_ms });
    }

    // Serialise the ASCII/JSON output
    let gif_output = GifOutput {
        font: font_label,
        width: char_width,
        height: char_height,
        frames: gif_frames,
    };

    let ascii_json = serde_json::to_string_pretty(&gif_output)
        .map_err(|e: serde_json::Error| JsValue::from_str(&e.to_string()))?;

    Ok(ConvertGifResult {
        ascii_json,
        original_gif: original_frames,
        processed_gif: processed_frames,
    })
}

fn rgba_image_to_png_bytes(img: &image::RgbaImage) -> Result<Vec<u8>, image::ImageError> {
    let mut buf = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut buf), image::ImageFormat::Png)?;
    Ok(buf)
}
