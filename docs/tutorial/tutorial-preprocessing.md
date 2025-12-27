# Tutorial: Preprocessing

There are more transformations you can do to the image before transforming it:

- `-n, --negative`
- `-r, --bw`
  - `-t, --threshold <THRESHOLD>`: threshold for the black and white.
- `-v, --visible`
- `-s, --saturate`
- `-g, --grayscale`

This are kind of self explanatory, but you can check out [how the app works](../how-it-works.md) to see how they work.

And there is a special one here:

- `-m, --midpoint-brightness <MIDPOINT_BRIGHTNESS>`: Because of [how the app works](../how-it-works.md) we need the brightness to have negative values, so we subtract this value to the brightness.

Higher values will have cloudier edges, and lower values will have sharper edges.

It can also be used to print images like this:

```bash
l2a palm.jpg -c -m 0 --center --chars "8"
```

![Palm with color](../../images/ascii/palm_m0_center_chars8.png)

We do not recommend using this for images with a lot of details, like this palm, this might be the only solution.