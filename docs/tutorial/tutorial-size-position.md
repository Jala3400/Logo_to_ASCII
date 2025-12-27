# Tutorial: Size and Position

## Adjusting size

There are multiple options to change the size of the final result:

- `-w, --wc <WIDTH_IN_CHARS>` This is number of characters the final result with have per line.
- `-h, --hc <HEIGHT_IN_CHARS>` This is the number of lines the final result will have.
- `--wp <WIDTH_IN_PIXELS>` This is is width in pixels you want the image to have.
- `--hp <HEIGHT_IN_PIXELS>` This is the height in pixels you want the image to have.

The size in characters overrides the size in pixels, and if you only change one dimension, the aspect ratio will be kept.

## Adjusting position.

Sometimes the image is not in the position we want. To fix it we can change the padding.

- `--center` Automatically adjusts the padding so the image is centered.
- `--padx <PADDING_X>` Specifies more padding to be added on the x axis.
- `--pady <PADDING_Y>` Specifies more padding to be added on the y axis.

The padding will be treated in the same way as a transparent color, and the padding you specify manually will be added to the one calculated on `--center`.