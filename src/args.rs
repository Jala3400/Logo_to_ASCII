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

    /// Sould make a distictnion between colors
    #[arg(short, long, default_value_t = false)]
    pub color: bool,

    /// Number of color the app processes when the -c flag is active (default 0)
    #[arg(short, long, default_value_t = 0)]
    pub num_colors: u16,
}