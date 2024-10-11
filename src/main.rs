use std::io;

use clap::Parser;
use logo_to_ascii::{abc, args::Args, proc_image::convert_image};

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