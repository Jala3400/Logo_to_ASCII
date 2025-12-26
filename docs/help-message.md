# Usage

## Command Line Interface

```
Usage: l2a.exe [OPTIONS] <PATH>
```

### Options
- `-V, --version`: Print version

### Help
- `--help`: Print the help message

### Input/Output
- `-o, --output <OUTPUT>`: Output the image to a file
- `-f, --font-name <FONT_NAME>`: Name of the font to use (optional)
- `--font-path <FONT_PATH>`: Path of the font to use (optional)
- `<PATH>`: Path of the image to process

### Character Set
- `--chars <CHARS>`: Characters used to convert the image [default: "8dbqp'·. "]
- `-a, --add-chars <ADD_CHARS>`: Add characters to the default ones [default: ]
- `-x, --except <EXCEPT>`: Exclude characters from the default ones [default: ]
- `--dict <DICTS>...`: Specify to use the built-in dictionary. Overrides `--chars` but not `--add-chars` and `--except` You can specify multiple times to use multiple dictionaries or specify a list separated by commas [possible values: all, symbols, blocks, blocks_all, box, box_all, box_double, box_double_all, nerd, math, numbers, letters]
- `--char-size <CHAR_SIZE>`: Font size to use [default: 16]

### Image Processing
- `-n, --negative`: Inverse the brightness of the image (transparent is never printed)
- `-r, --bw`: Preprocess the image to black and white. Makes the transparent pixels black by default
- `-t, --threshold <THRESHOLD>`: Threshold value for the black and white conversion (from 0 to 255) [default: 0.5]
- `-v, --visible`: Makes transparent pixels visible
- `-s, --saturate`: Saturate the image
- `-g, --grayscale`: Grayscale and brighten the image
- `-m, --midpoint-brightness <MIDPOINT_BRIGHTNESS>`: Midpoint of the brightness spectrum [default: 0.5]

### Dimensions and Padding
- `-w, --wc <WIDTH_IN_CHARS>`: Number of characters in the width of the end image (0 to default) [default: 0]
- `-h, --hc <HEIGHT_IN_CHARS>`: Number of characters in the height of the end image (0 to default) [default: 0]
- `--wp <WIDTH_IN_PIXELS>`: Actual width of the image (0 to default) [default: 0]
- `--hp <HEIGHT_IN_PIXELS>`: Actual height of the image (0 to default) [default: 0]
- `--padx <PADDING_X>`: Padding of the width of the image [default: 0]
- `--pady <PADDING_Y>`: Padding of the height of the image [default: 0]
- `--center`: Center the image in respect to the characters by adjusting the padding

### Borders and Colors
- `-b, --borders <BORDER_CRITERIA>`: Separates colors (change thickness with `-b`) [possible values: color, brightness, all]
- `-k, --thick <BORDER_THICKNESS>`: Detect borders measuring brightness (when not used with color) (0 to disable) [default: 0]
- `--color-diff <COLOR_DIFF>`: Threshold for the color difference (from 0 to 360, will be the remainder after division by 360) [default: 30]
- `--brightness-diff <BRIGHTNESS_DIFF>`: Threshold for the brightness difference (from 0 to 1) [default: 0.1]
- `-c, --print-color`: Print the image with colors

### Algorithm and Misc
- `--alg <ALGORITHM>`: Algorithm used to match blocks to characters [default: max_prod] [possible values: max_prod, min_diff, min_diff_sq, gradient, corr, ncc]
- `--verbose`: Print information about the image
