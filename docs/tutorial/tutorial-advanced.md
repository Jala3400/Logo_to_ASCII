# Tutorial: Advanced Techniques

This page covers advanced combinations of options and techniques for getting the most out of Logo to ASCII.

## Combining Multiple Options

All arguments can be mixed together for complex results. Here are some powerful combinations:

```bash
l2a .\images\Cross_Calatrava.png -cCv
```

![Cross of Calatrava with borders and colors](./images/ascii/cruz_cCv.png)

```bash
l2a .\images\tentacles.png -cCv -a "@#$&Yg*'´_/\ \""
```

![Tentacles with borders, colors, and custom characters](./images/ascii/tentaculos_cCv_chars.png)

## Saving Processed Images

Before converting to characters, the application processes the image. To save this intermediate result, use `-o <filename>`:

```bash
l2a .\images\tentacles.png -cCv -o final_tentacles_cCv.png
```

![Final processed tentacles image](./images/final/final_tentacles_cCv_chars.png)

## Advanced Character Management

### Using Built-in Character Sets

Instead of manually specifying characters, use built-in sets:

```bash
l2a .\images\Cross_Calatrava.png --dict blocks
```

Available sets: `all`, `symbols`, `blocks`, `blocks_all`, `box`, `box_all`, `box_double`, `box_double_all`, `nerd`, `math`, `numbers`, `letters`

### Custom Fonts

Change the font used for character comparison:

```bash
l2a .\images\Cross_Calatrava.png --font /path/to/font.ttf
```

> [!WARNING]
> `--font` doesn't adapt blocks to font size. Each character is assumed to be monospace with 1:2 proportions, which may distort results.

## Advanced Border Techniques

### Brightness-based Borders

For borders based on luminosity instead of color:

```bash
l2a .\images\image.png --borders brightness -k 2
```

> [!NOTE]
> Not recommended for images with colors of similar brightness (like yellow and white).

### Fine-tuning Border Detection

Adjust sensitivity:

```bash
l2a .\images\image.png --borders color --color-diff 45 --brightness-diff 0.2
```

## Image Preprocessing Pipeline

The preprocessing order is important:

1. Resize (`-w`, `-h`, `--wp`, `--hp`)
2. Add padding (`--padx`, `--pady`, `--center`)
3. Apply borders (`--borders`, `-k`)
4. Color adjustments (`-s`, `-g`, `--bw`, `-t`, `-n`, `-v`, `-m`)

## Tips for Complex Images

- Use `--verbose` to see image information
- Experiment with different algorithms for various image types
- Combine preprocessing steps strategically
- Save intermediate results with `-o` to debug your pipeline

For more options, see the [full usage reference](../usage.md).