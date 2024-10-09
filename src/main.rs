mod abc;
mod block;
mod calc_pixel;

use std::io;

use block::convert_image;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of the image to process
    #[arg(short, long)]
    path: String,

    /// Path of the font to use (optional)
    #[arg(short, long)]
    font: Option<String>,

    /// Path of the characters used (optional)
    #[arg(long)]
    chars: Option<String>,

    /// Inverse the colors of the image
    #[arg(short, long, default_value_t = false)]
    inverse: bool,

    /// Sould make a distictnion between colors
    #[arg(short, long, default_value_t = false)]
    color: bool,
}
fn main() -> io::Result<()> {
    let args: Args = Args::parse();

    // Load the image
    let img = image::open(&args.path).unwrap();

    let chars = if let Some(chars) = &args.chars {
        chars
    } else {
        "8dbqp'·. "
    };

    let font = abc::get_dict8x16(&args.font, chars);

    convert_image(img, args, font);

    Ok(())
}