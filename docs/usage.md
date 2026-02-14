# Usage

## Command Line Interface

```bash
Usage: l2a.exe [OPTIONS] <PATH>
```

### Options

- `-V, --version`: Print version

### Help

- `--help`: Print the help message

### Input/Output

- `-o, --output <OUTPUT>`: Output the image to a file
- `-f, --font-name <FONT_NAME>`: Name of the font to use (optional)
- `-F, --font-path <FONT_PATH>`: Path of the font to use (optional)
- `<PATH>`: Path of the image to process

### Character Set

- `--chars <CHARS>`: Characters used to convert the image [default: "8dbqp'·. "]
- `-a, --add-chars <ADD_CHARS>`: Add characters to the default ones [default: ]
- `-x, --except <EXCEPT>`: Exclude characters from the default ones [default: ]
- `-d, --dict <DICTS>...`: Use a built-in character set. Overrides `--chars` but not `--add-chars` and `--except`. You can specify multiple times or as a comma-separated list [possible values: default, all, symbols, blocks, blocks_all, box, box_all, box_double, box_double_all, nerd, math, numbers, letters]
- `--char-size <CHAR_SIZE>`: Font size to use [default: 16]

### Image Processing

- `-n, --negative`: Invert the brightness of the image
- `-B, --black-white`: Turn the image into black and white
- `-t, --threshold <THRESHOLD>`: Threshold value for the black and white conversion (from 0 to 1) [default: 0.5]
- `-v, --visible`: Make transparent pixels visible
- `-s, --saturate`: Saturate the pixels of the image while keeping the dark pixels dark
- `-g, --grayscale`: Convert the image to grayscale and normalize the brightness
- `-m, --midpoint-brightness <MIDPOINT_BRIGHTNESS>`: Midpoint of the brightness spectrum [default: 0.5]

### Dimensions and Padding

- `-w, --width-chars <WIDTH_IN_CHARS>`: Number of characters in the final image width
- `-h, --height-chars <HEIGHT_IN_CHARS>`: Number of characters in the final image height
- `-W, --width-pixels <WIDTH_IN_PIXELS>`: Width of the image in pixels
- `-H, --height-pixels <HEIGHT_IN_PIXELS>`: Height of the image in pixels
- `-P, --pad <PADDING>`: Padding of the image on all sides [default: 0]
- `-X, --padx <PADDING_X>`: Horizontal padding [default: 0]
- `-Y, --pady <PADDING_Y>`: Vertical padding [default: 0]
- `-C, --center`: Center the image in respect to the characters by adjusting the padding

### Borders and Colors

- `-c, --print-color`: Print the image with colors
- `-b, --borders <BORDER_CRITERIA>..`: Draws borders on the image according to the specified policy. You can specify the argument multiple times or as a comma separated list. [possible values: color, brightness, alpha, all]
- `-k, --thickness <BORDER_THICKNESS>`: Border thickness (default: width of the character)
- `--color-diff <COLOR_DIFF>`: Threshold for the color difference (from 0 to 360, modulus 360) [default: 30]
- `--brightness-diff <BRIGHTNESS_DIFF>`: Threshold for the brightness difference (from 0 to 1) [default: 0.1]
- `--alpha-diff <ALPHA_DIFF>`: Threshold for the alpha difference (from 0 to 1) [default: 0.0]

### Algorithm and Misc

- `-A, --alg <ALGORITHM>`: Algorithm used to match blocks to characters [default: max_prod] [possible values: max_prod, min_diff, min_diff_sq, gradient, corr, ncc]
- `--verbose`: Print information about the image
