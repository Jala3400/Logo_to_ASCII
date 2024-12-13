use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path of the image to process
    #[arg(short, long)]
    pub path: String,

    /// Path of the font to use (optional)
    #[arg(short, long)]
    pub font: Option<String>,

    /// Characters used to convert the image
    #[arg(long, default_value = "8dbqp'·. ")]
    pub chars: String,

    /// Add characters to the default ones
    #[arg(short, long, default_value = "")]
    pub add_chars: String,

    /// Use characters except:
    #[arg(short = 'x', long, default_value = "")]
    pub except: String,

    /// Use all ASCII pintable characters to convert the image
    #[arg(long, default_value_t = false)]
    pub all: bool,

    /// Inverse the brightness of the image (transparent is never printed)
    #[arg(short, long, default_value_t = false)]
    pub inverse: bool,

    /// Separates colors (change thickenss with `-b`)
    #[arg(short, long, default_value_t = false)]
    pub color: bool,

    /// Detect borders measuring brightness (when not used with color) (0 to disable)
    #[arg(short, long, default_value_t = 0)]
    pub border: u32,

    /// Preprocess the image to black and white.
    /// Makes the transparent pixels black by default.
    #[arg(short = 'r', long, default_value_t = false)]
    pub preprocess: bool,

    /// Threshold value for the black and white conversion (from 0 to 255)
    #[arg(short, long, default_value_t = 127)]
    pub threshold: u8,

    /// Threshold for the color difference (from 0 to 360)
    #[arg(short, long, default_value_t = 30)]
    pub difference: u16,

    /// Makes transparent pixels visible
    #[arg(short, long, default_value_t = false)]
    pub visible: bool,

    /// Number of characters in the width of the end image (0 to default)
    #[arg(short, long, default_value_t = 0)]
    pub width: u32,

    /// Number of characters in the height of the end image (0 to default)
    #[arg(short, long, default_value_t = 0)]
    pub height: u32,

    /// Actual width of the image (0 to default)
    #[arg(long = "aw", default_value_t = 0)]
    pub actual_width: u32,

    /// Actual height of the image (0 to default)
    #[arg(long = "ah", default_value_t = 0)]
    pub actual_height: u32,

    /// Midpoint of the brightness spectrum
    #[arg(short, long, default_value_t = 0.5)]
    pub midpoint_brightness: f32,

    /// Offsetx of the width of the image
    #[arg(long = "ofx", default_value_t = 0)]
    pub offsetx: usize,

    /// Offsety of the height of the image
    #[arg(long = "ofy", default_value_t = 0)]
    pub offsety: usize,

    /// Print the image with colors
    #[arg(short = 'C', long, default_value_t = false)]
    pub text_color: bool,

    /// Saturate the image
    #[arg(short = 's', long, default_value_t = false)]
    pub saturate: bool,

    /// Output the image to a file
    #[arg(short, long)]
    pub output: Option<String>,
}
