# Tutorial: Preprocessing

There are more transformations you can do to the image before transforming it:

-   `-n, --negative`
-   `-r, --bw`
    -   `-t, --threshold <THRESHOLD>`: threshold for the black and white.
-   `-v, --visible`
-   `-s, --saturate`
-   `-g, --grayscale`

This are kind of self explanatory, but you can check out [how the app works](../how-it-works.md) to see how they work.

There is also a special argument that doesn't fit anywhere else:

-   `-m, --midpoint-brightness <MIDPOINT_BRIGHTNESS>`: Because of [how the app works](../how-it-works.md) we need the brightness to have positive and negative values, so we subtract this value to the brightness.

Higher values will have cloudier edges, and lower values will have sharper edges.

It can also be used to print images like this:

```bash
l2a palm.jpg -c -m 0 --center --chars "8"
```

![Palm with color](../../images/ascii/palm_m0_center_chars8.png)

We do not recommend using this app for images with a lot of details, like this palm tree, but this is the best way to do it.
