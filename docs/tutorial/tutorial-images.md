# Tutorial: Working with Different Image Types

Logo to ASCII works best with logos and simple graphics, but you can still get results with other image types. This page covers tips for various image categories.

## Photos with Many Details

If you're converting a detailed photo to ASCII, manage your expectations:

> [!WARNING]
> ASCII characters don't have enough detail to make photos look good. This application is designed for wide surfaces and defined edges.

That said, here are some techniques to improve results:

### Basic Photo Conversion

Change the luminosity midpoint (e.g., to 0.7) or convert to black and white with threshold adjustment:

```bash
l2a .\images\palm.jpg --bw -t 0.7
```

![Palm in black and white](./images/palmera.png)

### Colored Photos

For color photos, use `-m 0` with `-C` to print all characters with colors:

```bash
l2a .\images\palm.jpg -m 0 -C
```

![Palm with colors](./images/palmera_m0_C.png)

## Best Practices by Image Type

### Logos and Icons
- Use default settings or add borders for multi-color logos
- Experiment with character sets for different styles
- Colors work well for brand recognition

### Line Art and Illustrations
- `--borders brightness` can enhance edges
- Try different algorithms like `gradient` for smooth shading
- Custom character sets can mimic drawing styles

### Text and Typography
- Use `--bw` with appropriate threshold
- Adjust character size to match font proportions
- Consider using specific character sets that match the text style

### Screenshots and UI Elements
- Borders can help separate UI components
- Use colors to maintain visual hierarchy
- Adjust midpoint for better contrast

## Image Preparation Tips

### Vector Graphics
- Convert SVG to high-resolution PNG before processing
- Use solid colors and defined edges
- Avoid gradients unless using `gradient` algorithm

### Photography
- Crop to focus on main subject
- Adjust brightness/contrast in image editor first
- Consider converting to line art for better results

### General Advice
- Start with high-resolution images (at least 1000px on longest side)
- Use images with good contrast
- Avoid busy backgrounds
- Test different combinations on small crops first

## When ASCII Art Isn't Suitable

Some images simply don't work well as ASCII art:
- Highly detailed photographs
- Images with subtle color gradients
- Complex scenes with many small elements
- Low-contrast images

In these cases, consider:
- Using the image as-is
- Creating custom ASCII art manually
- Using different art generation tools
- Focusing on simple logos or icons instead

For more image processing options, see the [usage reference](../usage.md).