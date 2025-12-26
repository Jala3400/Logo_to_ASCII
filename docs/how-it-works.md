# How It Works

The idea came from a video where an image was converted to ASCII. However, a lot of information was lost and the characters didn't have the shape they should.

This algorithm operates on pixels instead of blocks.

## 1. Characters

First, characters are processed. In the console, they have a 2:1 height to width ratio. Once a size is chosen (default 8x16), a bitmap of each character is made, containing the luminosity of each pixel.

When calculating luminosity, a value from 0 to 1 is obtained. It is important to subtract 0.5 to get negative and positive values.

Additionally, the number of pixels with positive luminosity is counted for future optimization.

There are several operations that allow changing characters:

- `--chars <characters>`: Changes the default character set. Single or double quotes can be used to include spaces.
- `--all`: Changes the default set to all printable ASCII characters (takes priority over chars).
- `-a <characters>`: Adds characters to the default set.
- `-x <characters>`: Removes characters from the default set.
- `--font <font_path>.ttf`: Changes the font used to compare characters.

## 2. Preprocessing (if any)

Before converting the image to text, the application allows several operations. They are, in order of execution:

- Change the image size:

  - `-w <character_width>`: Width in characters
  - `-h <character_height>`: Height in characters
  - `--wp <pixel_width>`: Width in pixels
  - `--ah <pixel_height>`: Height in pixels

- Add offset to the image:

  - `--ofx <offset_x>`: Offset in x
  - `--ofy <offset_y>`: Offset in y

- Saturate each pixel: `-s`
- Draw borders between colors:

  - `--borders <criteria>`: Border criteria.
  - `-k <thickness>`: Border thickness.
  - `--color-diff <diff>`: Color difference.
  - `--brightness-diff <diff>`: Brightness difference.

- Make the negative of the image (invert luminosity): `-n`

- Show transparent pixels: `-v`
- Grayscale: `-g`
- Convert image to black and white:

  - `--bw`: Converts to black and white.
  - `-t <threshold>`: Minimum value for a pixel to become white.

## 3. Convert Blocks to Character

Afterwards, the image is divided into blocks with the same measures as the characters and compared with all characters.

For each character, the luminosity of each pixel is multiplied by that of its counterpart in the block, and all values are summed (`[0][0] * [0][0] + [0][1] * [0][1] + ... + [n][m] * [n][m]`). Finally, the character with the highest score is printed.

In this section, the option to change the luminosity midpoint with `-m <midpoint>` is given. This argument defaults to 0.5 and indicates what is subtracted from each pixel's luminosity (between 0 and 1).

On the other hand, the option to use a different algorithm to establish the best-fitting character is given, such as the minimum difference, using `--algo <max_prod|min_diff|min_diff_sq|gradient|corr|ncc>`. `max_prod` is the default algorithm name.

The `min_diff` and `min_diff_sq` algorithms follow the principle of operating pixel by pixel, so results will be similar to `max_prod` but less defined, while `gradient` calculates the average luminosity of the block.

The `gradient` algorithm normalizes the luminosity of the characters, but not that of the image. This means that if you add or remove characters, the ranges assigned to each one change. However, if you put the same image darker, the brighter characters will not appear.

**Optimization:**

Only applies to `max_prod` when the first character is the space.

In this step, the number of illuminated pixels in the block is also counted. A character is only considered for printing if half of its positive luminosity pixels are at least the number of illuminated pixels in the block.

The sentence is complicated to understand, but in summary, if a character has 10 pixels with luminosity > 0, a character will only be considered if it has at least 5 illuminated pixels. If it had less than 5, there would be no scenario where that character would be chosen before the space. If there is no space, the character with the fewest illuminated pixels is chosen.

Additionally, if all pixels are completely illuminated, the brightest character can be printed directly.

The algorithm works because multiplying two positive values gives a positive number, and multiplying two negative numbers also. This rewards matches of pixels (and non-pixels) and penalizes differences.