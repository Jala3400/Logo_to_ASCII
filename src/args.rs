use crate::types::{Algorithm, BorderCriteria, BuiltInCharSet};
use clap::builder::styling::AnsiColor;
use clap::builder::Styles;
use clap::Parser;
use std::num::NonZeroU32;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = None,
    disable_help_flag = true,
    styles = Styles::styled()
        .header(AnsiColor::BrightGreen.on_default())
        .literal(AnsiColor::BrightCyan.on_default())
        .placeholder(AnsiColor::Cyan.on_default())
)]
pub struct Args {
    // Help
    /// Print the help message
    #[arg(long, action = clap::ArgAction::HelpLong, help_heading = "Help")]
    help: Option<bool>,

    // Input/Output
    /// Path of the image to process
    #[arg(help_heading = "Input/Output")]
    pub path: String,

    /// Output the image to a file
    #[arg(short, long, help_heading = "Input/Output")]
    pub output: Option<String>,

    /// Name of the font to use (optional)
    #[arg(short, long, help_heading = "Input/Output")]
    pub font_name: Option<String>,

    /// Path of the font to use (optional)
    #[arg(short = 'F', long, help_heading = "Input/Output")]
    pub font_path: Option<String>,

    // Character Set
    /// Characters used to convert the image
    #[arg(long, default_value = "8dbqp'·. ", help_heading = "Character Set")]
    pub chars: String,

    /// Add characters to the default ones
    #[arg(short, long, default_value = "", help_heading = "Character Set")]
    pub add_chars: String,

    /// Exclude characters from the default ones
    #[arg(short = 'x', long, default_value = "", help_heading = "Character Set")]
    pub except: String,

    /// Specify to use the built-in dictionary. Overrides `--chars` but not `--add-chars` and `--except`
    /// You can specify multiple times to use multiple dictionaries or specify a list separated by commas
    #[arg(short, long = "dict", num_args = 1.., value_delimiter = ',', value_enum, help_heading = "Character Set")]
    pub dicts: Option<Vec<BuiltInCharSet>>,

    /// Font size to use
    #[arg(long, default_value = "16", help_heading = "Character Set")]
    pub char_size: NonZeroU32,

    // Image Processing
    /// Inverse the brightness of the image
    #[arg(
        short,
        long,
        default_value_t = false,
        help_heading = "Image Processing"
    )]
    pub negative: bool,

    /// Turn the image into black and white.
    #[arg(
        short = 'B',
        long = "black-white",
        default_value_t = false,
        help_heading = "Image Processing"
    )]
    pub black_and_white: bool,

    /// Threshold value for the black and white conversion (from 0 to 1)
    #[arg(short, long, default_value_t = 0.5, help_heading = "Image Processing")]
    pub threshold: f32,

    /// Makes transparent pixels white instead of black
    #[arg(
        short,
        long,
        default_value_t = false,
        help_heading = "Image Processing"
    )]
    pub visible: bool,

    /// Saturate each pixel of the image while keeping the dark pixels dark
    #[arg(
        short = 's',
        long,
        default_value_t = false,
        help_heading = "Image Processing"
    )]
    pub saturate: bool,

    /// Grayscale and normalize the brightness of the image
    #[arg(
        short = 'g',
        long,
        default_value_t = false,
        help_heading = "Image Processing"
    )]
    pub grayscale: bool,

    /// Midpoint of the brightness spectrum
    #[arg(short, long, default_value_t = 0.5, help_heading = "Image Processing")]
    pub midpoint_brightness: f32,

    // Dimensions and Padding
    /// Number of characters in the width of the end image
    #[arg(
        short = 'w',
        long = "width-chars",
        help_heading = "Dimensions and Padding"
    )]
    pub width_in_chars: Option<NonZeroU32>,

    /// Number of characters in the height of the end image
    #[arg(
        short = 'h',
        long = "height-chars",
        help_heading = "Dimensions and Padding"
    )]
    pub height_in_chars: Option<NonZeroU32>,

    /// Width of the image in pixels
    #[arg(
        short = 'W',
        long = "width-pixels",
        help_heading = "Dimensions and Padding"
    )]
    pub width_in_pixels: Option<NonZeroU32>,

    /// Height of the image in pixels
    #[arg(
        short = 'H',
        long = "height-pixels",
        help_heading = "Dimensions and Padding"
    )]
    pub height_in_pixels: Option<NonZeroU32>,

    /// Padding of the image on all sides
    #[arg(
        short = 'P',
        long = "pad",
        default_value_t = 0,
        help_heading = "Dimensions and Padding"
    )]
    pub padding: usize,

    /// Horizontal padding
    #[arg(
        short = 'X',
        long = "padx",
        default_value_t = 0,
        help_heading = "Dimensions and Padding"
    )]
    pub padding_x: usize,

    /// Vertical padding
    #[arg(
        short = 'Y',
        long = "pady",
        default_value_t = 0,
        help_heading = "Dimensions and Padding"
    )]
    pub padding_y: usize,

    /// Center the image relative to the size of all the characters by adjusting the padding
    #[arg(
        short = 'C',
        long,
        default_value_t = false,
        help_heading = "Dimensions and Padding"
    )]
    pub center: bool,

    // Borders and Colors
    /// Print the image with colors
    #[arg(
        short = 'c',
        long,
        default_value_t = false,
        help_heading = "Borders and Colors"
    )]
    pub print_color: bool,

    /// Draws borders on the image according to the specified policy. You can specify multiple policies.
    #[arg(short, long = "borders", num_args = 1.., value_delimiter = ',', value_enum, help_heading = "Borders and Colors")]
    pub border_criteria: Option<Vec<BorderCriteria>>,

    /// Border thickness. (optional, default: width of the character)
    #[arg(short = 'k', long = "thickness", help_heading = "Borders and Colors")]
    pub border_thickness: Option<NonZeroU32>,

    /// Threshold for the color difference (from 0 to 360, will be the remainder after division by 360)
    #[arg(
        long = "color-diff",
        default_value_t = 30.0,
        help_heading = "Borders and Colors"
    )]
    pub color_diff: f32,

    /// Threshold for the brightness difference (from 0 to 1)
    #[arg(
        long = "brightness-diff",
        default_value_t = 0.1,
        help_heading = "Borders and Colors"
    )]
    pub brightness_diff: f32,

    /// Threshold for the alpha difference (from 0 to 1)
    #[arg(
        long = "alpha-diff",
        default_value_t = 0.0,
        help_heading = "Borders and Colors"
    )]
    pub alpha_diff: f32,

    // Algorithm and Misc
    /// Algorithm used to match blocks to characters
    #[arg(short = 'A', long = "alg", value_enum, default_value_t = Algorithm::MaxProd, help_heading = "Algorithm and Misc")]
    pub algorithm: Algorithm,

    /// Print information about the image
    #[arg(long, default_value_t = false, help_heading = "Algorithm and Misc")]
    pub verbose: bool,
}
