# Tutorial: Borders

This app has a simple border detection mechanism. You can see a more technical implementation in [how it works](../how-it-works.md#2-preprocessing-if-any).

Here is an explanation about which parameters can be used and some recommendations.

## Parameters

### Border policy

The argument `-b, --borders <BORDER_CRITERIA>..` decides how borders are identified. There are 3 possibilities:

-   `color`: Measures by hue difference.
-   `brightness`: Measures the brightness difference.
-   `alpha`: Measures the alpha difference.

And then there is the option `all`, which uses all of them.

It is recommended to use `all`, specially if dealing with transparencies.

If you want to use multiple criteria, but not all of them, you can write them separated by a comma, like this:

`-b color, brightness`. This will detect borders between colors and brightness, but not alpha.

### Border thickness

`-k, --thickness <BORDER_THICKNESS>`: Specifies the thickness of a border.

If not specified its default value is the width of a block.

### Thresholds

There is one threshold for color and another for brightness.

-   `--color-diff <COLOR_DIFF>`: Threshold for the color difference (from 0 to 360, modulus 360)
-   `--brightness-diff <BRIGHTNESS_DIFF>`: Threshold for the brightness difference (from 0 to 1)
-   `--alpha-diff <ALPHA_DIFF>`: Threshold for the brightness difference (from 0 to 1)

## Recommendations

If there are borders close to the actual borders of the image there might be artifacts. You should check out [repositioning](tutorial-size-position.md#practical-example).

If the image is symmetrical, usually a padding of 1 (with `-P 1`) can make the image look better. This is because the borders are detected by comparing a pixel with the one below and the one to its right, but you should always test.
