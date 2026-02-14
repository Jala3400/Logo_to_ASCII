# Tutorial: Preprocessing

There are more transformations you can apply to the image before transforming it:

- `-n, --negative` Invert the brightness of the image.
- `-B, --black-white` Turn the image into black and white
  - `-t, --threshold <THRESHOLD>`: Threshold for the black and white conversion.
- `-v, --visible` Make transparent pixels visible
- `-s, --saturate` Saturate the bright pixels and do the opposite to the dark ones.
- `-g, --grayscale` Convert the image to grayscale and normalizes the brightness (the darkest pixel will become black and the brightest will become white).

You can check out [how the app works](../how-it-works.md) to see a more detailed explanation.

There is also a special argument that doesn't fit anywhere else:

- `-m, --midpoint-brightness <MIDPOINT_BRIGHTNESS>`: Because of [how the app works](../how-it-works.md) we need the brightness to have positive and negative values, so we subtract this value from the brightness.

Higher values will have sharper edges and less colors from the image will be visible; and lower values will have cloudier edges, and more colors will be shown.

It can also be used to print images with a lot of details like [this](../tutorial.md#images-with-a-lot-of-details).
