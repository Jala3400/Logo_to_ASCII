# Tutorial: Preprocessing

There are more transformations you can do to the image before transforming it:

-   `-n, --negative` Inverts the brightness of the image.
-   `-r, --bw` Transforms the image into black and white
    -   `-t, --threshold <THRESHOLD>`: threshold for the black and white.
-   `-v, --visible` Makes the transparent pixels visible
-   `-s, --saturate` Saturates the color of bright pixels and makes the other ones darker.
-   `-g, --grayscale` Converts the image to grayscale and normalizes the brightness (the darkest pixel will become black and the lightest will become white).

You can check out [how the app works](../how-it-works.md) to see a more detailed explanation.

There is also a special argument that doesn't fit anywhere else:

-   `-m, --midpoint-brightness <MIDPOINT_BRIGHTNESS>`: Because of [how the app works](../how-it-works.md) we need the brightness to have positive and negative values, so we subtract this value to the brightness.

Higher values will have cloudier edges, and lower values will have sharper edges.

It can also be used to print images with a lot of details like [this](../tutorial.md#images-with-a-lot-of-details).
