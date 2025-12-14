use crate::types::Algorithm;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None, disable_help_flag = true)]
pub struct Args {
    // Help
    /// Print the help message
    #[arg(long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,

    // Input/Output
    /// Path of the image to process
    #[arg()]
    pub path: String,

    /// Output the image to a file
    #[arg(short, long)]
    pub output: Option<String>,

    /// Path of the font to use (optional)
    #[arg(short, long)]
    pub font: Option<String>,

    // Character Set
    /// Characters used to convert the image
    #[arg(long, default_value = "8dbqp'·. ")]
    pub chars: String,

    /// Add characters to the default ones
    #[arg(short, long, default_value = "")]
    pub add_chars: String,

    /// Use characters except:
    #[arg(short = 'x', long, default_value = "")]
    pub except: String,

    /// Use all ASCII printable characters to convert the image
    #[arg(long, default_value_t = false)]
    pub all: bool,

    // Image Processing
    /// Inverse the brightness of the image (transparent is never printed)
    #[arg(short, long, default_value_t = false)]
    pub negative: bool,

    /// Preprocess the image to black and white.
    /// Makes the transparent pixels black by default.
    #[arg(short = 'r', long = "bw", default_value_t = false)]
    pub black_and_white: bool,

    /// Threshold value for the black and white conversion (from 0 to 255)
    #[arg(short, long, default_value_t = 0.5)]
    pub threshold: f32,

    /// Makes transparent pixels visible
    #[arg(short, long, default_value_t = false)]
    pub visible: bool,

    /// Saturate the image
    #[arg(short = 's', long, default_value_t = false)]
    pub saturate: bool,

    /// Grayscale and brighten the image
    #[arg(short = 'g', long, default_value_t = false)]
    pub grayscale: bool,

    /// Midpoint of the brightness spectrum
    #[arg(short, long, default_value_t = 0.5)]
    pub midpoint_brightness: f32,

    // Dimensions and Offsets
    /// Number of characters in the width of the end image (0 to default)
    #[arg(short = 'w', long = "cw", default_value_t = 0)]
    pub char_width: u32,

    /// Number of characters in the height of the end image (0 to default)
    #[arg(short = 'h', long = "ch", default_value_t = 0)]
    pub char_height: u32,

    /// Actual width of the image (0 to default)
    #[arg(long = "pw", default_value_t = 0)]
    pub pixel_width: u32,

    /// Actual height of the image (0 to default)
    #[arg(long = "ph", default_value_t = 0)]
    pub pixel_height: u32,

    /// Offsetx of the width of the image
    #[arg(long = "ofx", default_value_t = 0)]
    pub offsetx: usize,

    /// Offsety of the height of the image
    #[arg(long = "ofy", default_value_t = 0)]
    pub offsety: usize,

    // Borders and Colors
    /// Separates colors (change thickness with `-b`)
    #[arg(short, long, default_value_t = false)]
    pub color_borders: bool,

    /// Detect borders measuring brightness (when not used with color) (0 to disable)
    #[arg(short, long, default_value_t = 0)]
    pub border: u32,

    /// Threshold for the color difference (from 0 to 360) (if used for brightness, it will be divided by 360 automatically)
    #[arg(short, long, default_value_t = 30)]
    pub difference: u16,

    /// Print the image with colors
    #[arg(short = 'C', long, default_value_t = false)]
    pub print_color: bool,

    // Algorithm and Misc
    /// Algorithm used to match blocks to characters
    #[arg(long = "algo", value_enum, default_value_t = Algorithm::MaxMult)]
    pub algorithm: Algorithm,

    /// Print information about the image
    #[arg(long, default_value_t = false)]
    pub verbose: bool,
}
