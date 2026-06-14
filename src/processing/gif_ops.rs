use crate::core::errors::L2aError;
use image::{codecs::gif::GifDecoder, AnimationDecoder, ImageDecoder, RgbaImage};
use std::io::Read;

/// A composited GIF frame ready for processing.
/// `delay` is kept for lossless GIF re-encoding; `delay_ms` is for players.
pub struct RawGifFrame {
    pub image: RgbaImage,
    pub delay_ms: u64,
    pub delay: image::Delay, // only needed by CLI GIF encoder
}

/// A fully processed GIF frame.
pub struct ProcessedGifFrame {
    pub ascii: String,
    pub image: RgbaImage,
    pub delay_ms: u64,
    pub delay: image::Delay,
}

/// Decode and composite all frames sequentially.
/// Compositing must be sequential because each frame may be a
/// partial update on top of the previous canvas.
pub fn composite_gif_frames<R: Read>(reader: R) -> Result<Vec<RawGifFrame>, L2aError> {
    let decoder = GifDecoder::new(reader)?;
    let (width, height) = decoder.dimensions();
    let mut canvas = RgbaImage::new(width, height);
    let mut frames = Vec::new();

    for frame_result in decoder.into_frames() {
        let raw = frame_result?;

        let (numer, denom) = raw.delay().numer_denom_ms();
        let delay_ms = if denom == 0 {
            0
        } else {
            numer as u64 / denom as u64
        };

        let x = raw.left() as u32;
        let y = raw.top() as u32;
        let fw = raw.buffer().width();
        let fh = raw.buffer().height();

        if x == 0 && y == 0 && fw == width && fh == height {
            canvas = raw.buffer().clone();
        } else {
            for py in 0..fh {
                for px in 0..fw {
                    let (cx, cy) = (x + px, y + py);
                    if cx < width && cy < height {
                        canvas.put_pixel(cx, cy, *raw.buffer().get_pixel(px, py));
                    }
                }
            }
        }

        frames.push(RawGifFrame {
            image: canvas.clone(),
            delay_ms,
            delay: raw.delay(),
        });
    }

    Ok(frames)
}
