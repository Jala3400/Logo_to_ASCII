# Tutorial: Size and Position

It is weird that the image that you have fits perfectly where you want and how you want it, so we provide a set of tools that help with it.

## Adjusting size

There are multiple options to change the size of the final result:

-   `-w, --wc <WIDTH_IN_CHARS>` This is number of characters the final result with have per line.
-   `-h, --hc <HEIGHT_IN_CHARS>` This is the number of lines the final result will have.
-   `--wp <WIDTH_IN_PIXELS>` This is is width in pixels you want the image to have.
-   `--hp <HEIGHT_IN_PIXELS>` This is the height in pixels you want the image to have.

Specifying size in characters overrides the size in pixels, and if you only change one dimension, the aspect ratio will be kept.

## Adjusting position

Sometimes the image is not in the position we want. To fix it we can change the padding.

-   `--center` Automatically adjusts the padding so the image is centered.
-   `--padx <PADDING_X>` Specifies more padding to be added on the x axis.
-   `--pady <PADDING_Y>` Specifies more padding to be added on the y axis.

The padding will be treated in the same way as a transparent color, and the padding you specify manually will be added to the one calculated on `--center`.

## Practical example

Lets revisit the example of the tentacles:

![Tentacles](../../images/sources/tentacles.png)

This image fill the characters perfectly, there is no need to center it, but when we render it we might notice a character that seems out of place on the left part of the photo:

```bash
l2a '.\images\tentacles.png' -b all -c
```

![Tentacles](../../images/ascii/tentacles_b_all_c.png)

That letter `d` is sticking a bit too much.

Lets have a look behind the scenes by saving the final image.

```bash
l2a '.\images\tentacles.png' -b all -c -o test.png
```

![Tentacles final](../../images/final/tentacles_b_all.png)

The border of the red tentacle is touching the left border of the image, so no border is detected there. The same thing happens with the blue tentacle on the right border.

We can fix this by adding padding in the x axis.

```bash
l2a '.\images\tentacles.png' -b all -c --padx 1 -o
```

![Tentacles padx1](../../images/ascii/tentacles_b_all_c_padx1.png)

The `d` has disappeared. Lets check the final image again

```bash
l2a '.\images\tentacles.png' -b all -c --padx 1 -o test2.png
```

![Tentacles final padx1](../../images/final/tentacles_b_all_padx1.png)

We can see that those borders have been detected.

However, if you look at the bottom there are still borders that have not been detected. They do not look really important, so fixing it is left as an exercise to the reader.
