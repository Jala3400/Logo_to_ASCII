use clap::ValueEnum;

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
/// * `width` - The width of each character cell in pixels
/// * `height` - The height of each character cell in pixels
pub struct FontBitmap {
    pub data: Vec<CharInfo>, // It is ordered by the min value, from highest to lowest
    pub width: usize,
    pub height: usize,
    pub vertical_step: usize,
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
        while i < self.data.len() && self.data[i].avg_brightness < char_info.avg_brightness {
            i += 1;
        }

        // If the character already exists, do not insert it again
        if i < self.data.len() && self.data[i].char == char_info.char {
            return;
        }

        self.data.insert(i, char_info);
    }

    /// Returns the total number of pixels in each character cell
    pub fn cell_size(&self) -> usize {
        self.width * self.height
    }
}

/// Represents the information of a character
///
/// # Fields:
///
/// * char - The character itself
/// * data - The bitmap of the character as a vector of brightness values
/// * min - The minimum brightness threshold for this character, calculated as half of the total bright blocks
/// * avg_brightness - The average brightness of the character, used for gradient-based algorithms. Ranges from 0 to 1.
/// * norm - The L2 norm (magnitude) of the character data, used for NCC algorithm
/// * mean - The mean brightness of the character data, used for correlation algorithm
/// * std - The standard deviation of the character data, used for correlation algorithm
pub struct CharInfo {
    pub char: char,
    pub data: Vec<f32>,
    pub min: usize,
    pub avg_brightness: f32,
    pub norm: f32,
    pub mean: f32,
    pub std: f32,
}

/// Criteria for detecting borders in the image.
///
/// This enum defines the different criteria that can be used to identify borders
/// in the image during the ASCII art generation process.
///
/// # Variants
///
/// * `Color` - Detect borders based on color differences.
/// * `Brightness` - Detect borders based on brightness differences.
/// * `Both` - Detect borders using both color and brightness differences.
#[derive(Debug, Clone, ValueEnum)]
pub enum BorderCriteria {
    #[value(name = "color")]
    Color,
    #[value(name = "brightness")]
    Brightness,
    #[value(name = "all")]
    All,
}

/// Built in character sets for ASCII art generation.
///
/// This enum defines different predefined character sets that can be used
/// to generate ASCII art from bitmap images.
///
/// # Variants
///
/// * `All` - All printable ASCII characters from 32 to 126
/// * `Symbols` - A small set of symbols
/// /// There is no good monospace font with braille characters included by default
/// /// Might work on this later
/// /// * `Braille` - A set of Braille characters for detailed patterns.
/// * `Blocks` - A set of block characters for more solid representations.
/// * `BlocksAll` - A larger set of block characters including partial blocks.
/// * `Box` - A set of box drawing characters for line-based art.
/// * `BoxAll` - A larger set of box drawing characters including diagonal lines.
/// * `BoxDouble` - A set of double-line box drawing characters.
/// * `BoxDoubleAll` - A larger set of double-line box drawing characters including diagonal lines.
/// * `Nerd` - A set of Nerd Font characters for enhanced visual detail.
/// * `Math` - A set of mathematical symbols.
/// * `Numbers` - A set of numeric characters (0-9).
/// * `Letters` - A set of alphabetic characters (A-Z, a-z).
#[derive(Debug, Clone, ValueEnum)]
pub enum BuiltInCharSet {
    #[value(name = "all")]
    All,
    #[value(name = "symbols")]
    Symbols,
    #[value(name = "blocks")]
    // #[value(name = "braille")]
    // Braille,
    Blocks,
    #[value(name = "blocks_all")]
    BlocksAll,
    #[value(name = "box")]
    Box,
    #[value(name = "box_all")]
    BoxAll,
    #[value(name = "box_double")]
    BoxDouble,
    #[value(name = "box_double_all")]
    BoxDoubleAll,
    #[value(name = "nerd")]
    Nerd,
    #[value(name = "math")]
    Math,
    #[value(name = "numbers")]
    Numbers,
    #[value(name = "letters")]
    Letters,
}

/// Algorithm enumeration for ASCII art generation methods.
///
/// This enum defines different algorithms that can be used to convert
/// bitmap images to ASCII art by matching pixel patterns with character bitmaps.
///
/// # Variants
///
/// * `MaxProd` - Uses maximum product algorithm for character matching
/// * `MinDiff` - Uses minimum difference algorithm to find the best character match
/// * `MinDiffSq` - Uses minimum squared difference algorithm for more precise character matching
/// * `Gradient` - Uses the average brightness of the block to find the closest character match
/// * `Correlation` - Uses Pearson correlation coefficient to find the most correlated character pattern
/// * `Ncc` - Uses Normalized Cross-Correlation to match both pattern structure and brightness level
#[derive(Debug, Clone, ValueEnum)]
pub enum Algorithm {
    #[value(name = "max_prod")]
    MaxProd,
    #[value(name = "min_diff")]
    MinDiff,
    #[value(name = "min_diff_sq")]
    MinDiffSq,
    #[value(name = "gradient")]
    Gradient,
    #[value(name = "corr")]
    Correlation,
    #[value(name = "ncc")]
    Ncc,
}
