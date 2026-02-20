use crate::types::{Algorithm, BorderCriteria, BuiltInCharSet, OutputFormat};
use std::num::NonZeroU32;

/// Plain configuration struct used by the core processing pipeline.
///
/// Unlike [`crate::args::Args`], this struct has no dependency on `clap` and
/// can be constructed programmatically (e.g. from a web frontend via WASM).
/// All CLI-specific fields (`path`, `output`) live only in `Args`.
pub struct ImageConfig {
    // Font
    pub font_name: Option<String>,
    pub font_path: Option<String>,

    // Output
    pub format: OutputFormat,

    // Character set
    pub chars: String,
    pub add_chars: String,
    pub except: String,
    pub dicts: Option<Vec<BuiltInCharSet>>,
    pub char_size: NonZeroU32,

    // Image processing
    pub negative: bool,
    pub black_and_white: bool,
    pub threshold: f32,
    pub transparent_color: [u8; 3],
    pub saturate: bool,
    pub grayscale: bool,
    pub midpoint_brightness: f32,

    // Dimensions and padding
    pub width_in_chars: Option<NonZeroU32>,
    pub height_in_chars: Option<NonZeroU32>,
    pub width_in_pixels: Option<NonZeroU32>,
    pub height_in_pixels: Option<NonZeroU32>,
    pub padding: usize,
    pub padding_x: usize,
    pub padding_y: usize,
    pub center: bool,

    // Borders and colors
    pub print_color: bool,
    pub border_criteria: Option<Vec<BorderCriteria>>,
    pub border_thickness: Option<NonZeroU32>,
    pub border_color: [u8; 4],
    pub color_diff: f32,
    pub brightness_diff: f32,
    pub alpha_diff: f32,

    // Algorithm and misc
    pub algorithm: Algorithm,
    pub verbose: bool,
}

impl Default for ImageConfig {
    fn default() -> Self {
        ImageConfig {
            font_name: None,
            font_path: None,
            format: OutputFormat::Ansi,
            chars: "8dbqp'·. ".to_string(),
            add_chars: String::new(),
            except: String::new(),
            dicts: None,
            char_size: NonZeroU32::new(16).unwrap(),
            negative: false,
            black_and_white: false,
            threshold: 0.5,
            transparent_color: [0, 0, 0],
            saturate: false,
            grayscale: false,
            midpoint_brightness: 0.5,
            width_in_chars: None,
            height_in_chars: None,
            width_in_pixels: None,
            height_in_pixels: None,
            padding: 0,
            padding_x: 0,
            padding_y: 0,
            center: false,
            print_color: false,
            border_criteria: None,
            border_thickness: None,
            border_color: [0, 0, 0, 255],
            color_diff: 30.0,
            brightness_diff: 0.1,
            alpha_diff: 0.0,
            algorithm: Algorithm::MaxProd,
            verbose: false,
        }
    }
}

impl From<crate::args::Args> for ImageConfig {
    fn from(args: crate::args::Args) -> Self {
        ImageConfig {
            font_name: args.font_name,
            font_path: args.font_path,
            format: args.format,
            chars: args.chars,
            add_chars: args.add_chars,
            except: args.except,
            dicts: args.dicts,
            char_size: args.char_size,
            negative: args.negative,
            black_and_white: args.black_and_white,
            threshold: args.threshold,
            transparent_color: args.transparent_color,
            saturate: args.saturate,
            grayscale: args.grayscale,
            midpoint_brightness: args.midpoint_brightness,
            width_in_chars: args.width_in_chars,
            height_in_chars: args.height_in_chars,
            width_in_pixels: args.width_in_pixels,
            height_in_pixels: args.height_in_pixels,
            padding: args.padding,
            padding_x: args.padding_x,
            padding_y: args.padding_y,
            center: args.center,
            print_color: args.print_color,
            border_criteria: args.border_criteria,
            border_thickness: args.border_thickness,
            border_color: args.border_color,
            color_diff: args.color_diff,
            brightness_diff: args.brightness_diff,
            alpha_diff: args.alpha_diff,
            algorithm: args.algorithm,
            verbose: args.verbose,
        }
    }
}
