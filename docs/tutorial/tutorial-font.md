# Tutorial: Font

When rendering text each font has different measures for its characters, and, depending on the scale, each character is drawn differently.

## Changing the font

To change the font you have two arguments:

-   `-f, --font-name <FONT_NAME>`: Search a font by the font name. For example `-f "Cascadia Mono"`.
-   `-F, --font-path <FONT_PATH>`: Uses the font given its path.

This will automatically adjust the size of the block to match the font.

## Adjusting the scale.

`--char-size <CHAR_SIZE>` Lets you specify the desired height in pixels the characters will have.
