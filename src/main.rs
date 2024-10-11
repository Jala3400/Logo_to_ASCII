use std::io;

use clap::Parser;
use logo_to_ascii::{abc, args::Args, proc_image::convert_image};

fn main() -> io::Result<()> {
    let args: Args = Args::parse();

    // Load the image
    let img = image::open(&args.path).unwrap();

    let font = abc::get_dict8x16(&args.font, &args.chars);

    convert_image(img, args, font);

    Ok(())
}