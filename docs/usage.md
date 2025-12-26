# Advanced Usage

## Character Operations

The default character set is `8dbqp '·.`.

- `--chars <characters>`: Changes the default character set. Single or double quotes can be used to include spaces.
- `--all`: Changes the default set to all printable ASCII characters (takes priority over chars).
- `-a <characters>`: Adds characters to the default set.
- `-x <characters>`: Removes characters from the default set.
- `--font <font_path>.ttf`: Changes the font used to compare characters.
- `--dict <dict>`: Uses a built-in character set (all, symbols, blocks, etc.).

## Image Size

- `-w <character_width>`: Width in characters
- `-h <character_height>`: Height in characters
- `--wp <pixel_width>`: Width in pixels
- `--hp <pixel_height>`: Height in pixels
- `--padx <padding_x>`: Padding in x
- `--pady <padding_y>`: Padding in y
- `--center`: Centers the image

## Borders

- `--borders <criteria>`: Border criteria (color, brightness, all)
- `-k <thickness>`: Border thickness
- `--color-diff <diff>`: Color difference threshold
- `--brightness-diff <diff>`: Brightness difference threshold

## Other Preprocessing

- `-s`: Saturates each pixel to the maximum.
- `-n`: Inverts the image luminosity (negative).
- `-v`: Shows transparent pixels.
- `-g`: Converts to grayscale and brightens.
- `--bw`: Converts to black and white.
- `-t <threshold>`: Threshold for black and white.
- `-m <midpoint>`: Luminosity midpoint.

## Output

- `-c`: Prints with colors.
- `--algo <algorithm>`: Algorithm (max_prod, min_diff, etc.)
- `-o <output_image>`: Saves processed image.
- `--verbose`: Prints image info.

## Algorithms

- `max_prod`: Default. Multiplies luminosities pixel by pixel and sums.
- `min_diff`: Uses minimum difference.
- `min_diff_sq`: Uses squared minimum difference.
- `gradient`: Uses average luminosity of the block.
- `corr`: Correlation.
- `ncc`: Normalized cross-correlation.

The `gradient` algorithm normalizes character luminosities but not the image's. Adding/removing characters changes ranges.

Use `-g` to normalize the image.

## Saving Text

Redirect output: `> file.txt`