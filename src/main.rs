use clap::Parser;
use l2a::{args::Args, config::ImageConfig, errors::L2aError, process_image};

fn main() {
    if let Err(e) = run() {
        eprintln!("\n{}\n", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), L2aError> {
    // Parse the command line arguments
    let args = Args::parse();

    // Extract CLI-only fields before consuming args
    let path = args.path.clone();
    let output = args.output.clone();

    // Load the image
    let img = image::open(&path)?.to_rgba8();

    // Convert CLI args into the core config and run the pipeline
    let config = ImageConfig::from(args);
    let (ascii, processed_img) = process_image(img, config)?;

    // Print the ASCII art
    print!("{}", ascii);

    // Optionally save the processed image
    if let Some(ref output_path) = output {
        let path = std::path::Path::new(output_path);

        match image::ImageFormat::from_path(path) {
            Ok(format) => processed_img.save_with_format(output_path, format),
            Err(_) => processed_img
                .save_with_format(output_path.to_owned() + ".png", image::ImageFormat::Png),
        }
        .map_err(L2aError::Image)?;
    }

    Ok(())
}
