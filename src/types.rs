use clap::ValueEnum;

/// A structure representing a bitmap image with brightness values.
///
/// # Fields
///
/// * `data` - A vector containing brightness values of each pixel in the image
/// * `width` - The width of the image in pixels
/// * `height` - The height of the image in pixels
/// * `max_brightness` - The maximum brightness value found in the image data
pub struct Bitmap {
    pub data: Vec<f32>,
    pub width: usize,
    pub height: usize,
    pub max_brightness: f32,
}

/// A structure representing a bitmap font, containing character information
///
/// The font bitmap stores character data in a vector, ordered by their minimum
/// pixel value from highest to lowest. This ordering is crucial for the
/// ASCII art generation process.
///
/// # Fields
///
/// * `data` - A vector of character information entries ([`CharInfo`]),
///            ordered by minimum pixel value in descending order
pub struct FontBitmap {
    pub data: Vec<CharInfo>, // It is ordered by the min value, from highest to lowest
}

/// Inserts a character information into the bitmap in ascending order based on minimum value.
///
/// The character is inserted at the position where its minimum value fits in the ordered sequence.
/// If multiple characters have the same minimum value, the new character is inserted after them.
///
/// # Arguments
///
/// * `char_info` - The character information to be inserted into the bitmap
impl FontBitmap {
    pub fn insert_ord(&mut self, char_info: CharInfo) {
        let mut i = 0;
        while i < self.data.len() && self.data[i].min < char_info.min {
            i += 1;
        }
        self.data.insert(i, char_info);
    }
}

/// Represents the information of a character
///
/// # Fields:
///
/// * char - The character itself
/// * data - The bitmap of the character, represented as an array of 8x16 elements
/// * min - The minimum brightness threshold for this character, calculated as half of the total bright blocks
pub struct CharInfo {
    pub char: char,
    pub data: [f32; 8 * 16],
    pub min: usize,
}

/// Algorithm enumeration for ASCII art generation methods.
///
/// This enum defines different algorithms that can be used to convert
/// bitmap images to ASCII art by matching pixel patterns with character bitmaps.
///
/// # Variants
///
/// * `MaxMult` - Uses maximum multiplication algorithm for character matching
/// * `MinDiff` - Uses minimum difference algorithm to find the best character match
/// * `MinDiffSq` - Uses minimum squared difference algorithm for more precise character matching
#[derive(Debug, Clone, ValueEnum)]
pub enum Algorithm {
    #[value(name = "max_mult")]
    MaxMult,
    #[value(name = "min_diff")]
    MinDiff,
    #[value(name = "min_diff_sq")]
    MinDiffSq,
}
