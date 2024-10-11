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

    /// Path of the characters used (optional)
    #[arg(long)]
    pub chars: Option<String>,

    /// Inverse the colors of the image
    #[arg(short, long, default_value_t = false)]
    pub inverse: bool,

    /// Sould make a distictnion between colors
    #[arg(short, long, default_value_t = false)]
    pub color: bool,
}