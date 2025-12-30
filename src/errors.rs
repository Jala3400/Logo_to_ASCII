use thiserror::Error;

#[derive(Error, Debug)]
pub enum L2aError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Image error: {0}")]
    Image(#[from] image::ImageError),
    #[error("Font loading error: {0}")]
    Font(String),
    #[error("No characters available for conversion")]
    NoCharacters,
}
