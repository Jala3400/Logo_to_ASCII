# Tutorial: Algorithms

In addition to the default algorithm (`max_prod`), Logo to ASCII supports several other algorithms for matching image blocks to characters. Each algorithm has different strengths and can produce varying results.

## Available Algorithms

- `max_prod` (default)
- `min_diff`
- `min_diff_sq`
- `gradient`
- `corr`
- `ncc`

## Algorithm Explanations

The `min_diff` and `min_diff_sq` algorithms operate pixel by pixel, producing results similar to `max_prod` but with less definition.

The `gradient` algorithm calculates the average luminosity of each block rather than comparing shapes. This is more like traditional ASCII art converters.

`corr` and `ncc` measure correlation and normalized cross-correlation respectively. These consider how similar the variations are between a block and a character, with `ncc` also accounting for magnitude differences.

## Gradient Algorithm Details

The `gradient` algorithm normalizes character luminosities but not the image's. This means adding or removing characters changes the ranges assigned to each character. For darker images, brighter characters may not appear.

To fix this, use the `-g` flag, which converts the image to grayscale and brightens it so the brightest point becomes white.

## Examples

Using a gradient image to demonstrate:

```bash
l2a .\images\gradient.jpg -w 80 --algo gradient -a aeou
```

![Normal gradient](./images/gradient_aeou.png)

By adding an @ and removing the space, darker blocks use the next darker character and brighter blocks use brighter characters:

```bash
l2a .\images\gradient.jpg -w 80 --algo gradient -a ouae@ -x " "
```

![Adjusted gradient](./images/gradient_aeou@.png)

## Choosing an Algorithm

- Use `max_prod` for most logos and simple shapes (default)
- Try `gradient` for photos or images with smooth gradients
- Experiment with `corr` or `ncc` for complex patterns

For more algorithm details, see the [How It Works](../how-it-works.md) page.