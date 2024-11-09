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

    /// Use all ASCII pintable characters to convert the image
    #[arg(long, default_value_t = false)]
    pub all: bool,

    /// Inverse the colors of the image (transparent is never printed)
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
    #[arg(short, long, default_value_t = 64)]
    pub threshold: u8,

    /// Threshold for the color difference (from 0 to 360)
    #[arg(short, long, default_value_t = 30)]
    pub difference: u16,

    /// Makes transparent pixels visible
    #[arg(short, long, default_value_t = false)]
    pub visible: bool,
}
