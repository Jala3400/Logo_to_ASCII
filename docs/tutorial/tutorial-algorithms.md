# Tutorial: Algorithms

In addition to the default algorithm (`max_prod`), Logo to ASCII supports several other algorithms for matching image blocks to characters.

The best one is the default one by far (except for `cnn`, which acts in a really similar way), but we include them to experiment.

## Available Algorithms

- `max_prod` (default)
- `min_diff`
- `min_diff_sq`
- `gradient`
- `corr`
- `ncc`

## Algorithm Explanations

All of the algorithms work pixel by pixel except the gradient one.

The `min_diff` and `min_diff_sq` produce results similar to `max_prod` but less defined.

`corr` and `ncc` measure correlation and normalized cross-correlation respectively. These consider how similar the variations are between a block and a character, with `ncc` also accounting for magnitude differences.

The `gradient` algorithm calculates the average brightness of each block rather than comparing shapes. This is closer to traditional ASCII art converters.

You can see a more technical explanation on the algorithms in [this](../how-it-works.md#other-algorithms) page.

## Gradient Algorithm Details

Even if it operates with the average brightness of a block, it still has a few advantages over classical converters:

The `gradient` algorithm normalizes the brightness of the characters, so the darkest character has a brightness o 0 and the brightest a 1. However, it doesn't normalize the image.

This means that adding or removing characters changes the ranges of brightness assigned to each character, but that in dark images the bright characters might not appear.

To normalize the brightness of the image you should use the `-g, --grayscale` flag, which converts the image to grayscale and normalizes the brightness, so the brightest point in the image becomes white and the darkest black.

## Examples

Using a gradient image to demonstrate:

(The `--transparent-color FFF` flag is added to make the transparent pixels white, because the image doesn't fill perfectly the bottom part)

```bash
l2a .\images\sources\gradient.jpg -w 80 --alg gradient -a aeou --transparent-color FFF
```

![Normal gradient](../../images/ascii/gradient_aeou.png)

By adding an @ and removing the space, darker blocks use the next darker character, and brighter blocks use brighter characters:

```bash
l2a .\images\sources\gradient.jpg -w 80 --alg gradient -a ouae@ -x " " --transparent-color FFF
```

![Adjusted gradient](../../images/ascii/gradient_aeou@.png)

We can see that the lighter part of the image is now occupied by the character `@` instead of the `8`.

## Choosing an Algorithm

- Use `max_prod`
- Try `gradient` for photos or images with smooth gradients
- Experiment with the other ones, but with little hope

For more algorithm details, see the [How It Works](../how-it-works.md) page.
