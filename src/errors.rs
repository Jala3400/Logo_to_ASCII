use thiserror::Error;

#[derive(Error, Debug)]
pub enum L2aError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to process image: {0}")]
    Image(#[from] image::ImageError),
    #[error("Failed to load font: {0}")]
    Font(String),
    #[error("No characters available for conversion. Provide characters and check if they are supported by the font.")]
    NoCharacters,
}
