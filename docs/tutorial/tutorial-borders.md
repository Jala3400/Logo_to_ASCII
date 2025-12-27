# Tutorial: Borders

This app has a simple border detection mechanism. You can see a more technical implementation in [how it works](../how-it-works.md#2-preprocessing-if-any). 

Here is an explanation about which parameters you can use and some recommendations.

## Parameters

### Border policy

The argument `-b, --borders <BORDER_CRITERIA>` decides how borders are identified. There are 3:

- `color`: Measures by hue difference.
- `brightness`: Measures the brightness difference.
- `all`: Checks for both of them.

It is recommended to use `all`, specially if working with transparent images.

When working with transparent images, the first thing the app does it convert the transparent pixels to black (or white with the `-v, --visible` flag), and if you calculate the hue of the black color it is the same as the red color, so it will not detect borders there.

Something similar happens when working with black logos, but you can detect the borders by using the `-v, --visible` flag. You can pair it with the `-n, --negative` flag to invert the brightness and print the logo.

### Border thickness

`-k, --thick <BORDER_THICKNESS>` receives a number. It the thickness of the borders that will be drawn.

If not specified its default value is the width of a block.

### Thresholds

There is one threshold for color and other for brightness.

- `--color-diff <COLOR_DIFF>`: From 0 to 360, if the value is greater than 360 it will be the remainder.
- `--brightness-diff <BRIGHTNESS_DIFF>`: From 0 to 1, it is a float.