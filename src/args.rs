use crate::types::{Algorithm, BorderCriteria};
use clap::builder::styling::AnsiColor;
use clap::builder::Styles;
use clap::Parser;

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
    #[arg(long, help_heading = "Input/Output")]
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

    /// Use all ASCII printable characters to convert the image
    #[arg(long, default_value_t = false, help_heading = "Character Set")]
    pub all: bool,

    /// Font size to use
    #[arg(long, default_value_t = 16, help_heading = "Character Set")]
    pub char_size: u32,

    // Image Processing
    /// Inverse the brightness of the image (transparent is never printed)
    #[arg(
        short,
        long,
        default_value_t = false,
        help_heading = "Image Processing"
    )]
    pub negative: bool,

    /// Preprocess the image to black and white.
    /// Makes the transparent pixels black by default.
    #[arg(
        short = 'r',
        long = "bw",
        default_value_t = false,
        help_heading = "Image Processing"
    )]
    pub black_and_white: bool,

    /// Threshold value for the black and white conversion (from 0 to 255)
    #[arg(short, long, default_value_t = 0.5, help_heading = "Image Processing")]
    pub threshold: f32,

    /// Makes transparent pixels visible
    #[arg(
        short,
        long,
        default_value_t = false,
        help_heading = "Image Processing"
    )]
    pub visible: bool,

    /// Saturate the image
    #[arg(
        short = 's',
        long,
        default_value_t = false,
        help_heading = "Image Processing"
    )]
    pub saturate: bool,

    /// Grayscale and brighten the image
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
    /// Number of characters in the width of the end image (0 to default)
    #[arg(
        short = 'w',
        long = "wc",
        default_value_t = 0,
        help_heading = "Dimensions and Padding"
    )]
    pub width_in_chars: u32,

    /// Number of characters in the height of the end image (0 to default)
    #[arg(
        short = 'h',
        long = "hc",
        default_value_t = 0,
        help_heading = "Dimensions and Padding"
    )]
    pub height_in_chars: u32,

    /// Actual width of the image (0 to default)
    #[arg(
        long = "wp",
        default_value_t = 0,
        help_heading = "Dimensions and Padding"
    )]
    pub width_in_pixels: u32,

    /// Actual height of the image (0 to default)
    #[arg(
        long = "hp",
        default_value_t = 0,
        help_heading = "Dimensions and Padding"
    )]
    pub height_in_pixels: u32,

    /// Padding of the width of the image
    #[arg(
        long = "padx",
        default_value_t = 0,
        help_heading = "Dimensions and Padding"
    )]
    pub padding_x: usize,

    /// Padding of the height of the image
    #[arg(
        long = "pady",
        default_value_t = 0,
        help_heading = "Dimensions and Padding"
    )]
    pub padding_y: usize,

    /// Center the image in respect to the characters by adjusting the padding
    #[arg(long, default_value_t = false, help_heading = "Dimensions and Padding")]
    pub center: bool,

    // Borders and Colors
    /// Separates colors (change thickness with `-b`)
    #[arg(short, long = "borders", help_heading = "Borders and Colors")]
    pub border_criteria: Option<BorderCriteria>,

    /// Detect borders measuring brightness (when not used with color) (0 to disable)
    #[arg(
        short = 'k',
        long = "thick",
        default_value_t = 0,
        help_heading = "Borders and Colors"
    )]
    pub border_thickness: u32,

    /// Threshold for the color difference (from 0 to 360, will be the remainder after division by 360)
    #[arg(
        long = "color-diff",
        default_value_t = 30,
        help_heading = "Borders and Colors"
    )]
    pub color_diff: u16,

    /// Threshold for the brightness difference (from 0 to 1)
    #[arg(
        long = "brightness-diff",
        default_value_t = 0.1,
        help_heading = "Borders and Colors"
    )]
    pub brightness_diff: f32,

    /// Print the image with colors
    #[arg(
        short = 'c',
        long,
        default_value_t = false,
        help_heading = "Borders and Colors"
    )]
    pub print_color: bool,

    // Algorithm and Misc
    /// Algorithm used to match blocks to characters
    #[arg(long = "algo", value_enum, default_value_t = Algorithm::MaxProd, help_heading = "Algorithm and Misc")]
    pub algorithm: Algorithm,

    /// Print information about the image
    #[arg(long, default_value_t = false, help_heading = "Algorithm and Misc")]
    pub verbose: bool,
}
