# Tutorial: Font

When rendering text each font has different measures, and, depending on the scale, each character is drawn differently.

## Changing the font

To change the font you have two arguments:

- `-f, --font-name <font_name>`: Search a font by the font name. For example `-f "Cascadia Mono"`.
- `--font-path <font_path>`: Uses the font given its path.

This will automatically adjust the size of the block to match the font.

## Adjusting the scale.

`--char-size <height_in_pixels>` Lets you specify the desired height the characters will have.