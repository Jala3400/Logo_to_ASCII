# How It Works

The idea came from a video where an image was converted to ASCII. However, a lot of information was lost, and the characters didn't have the shape they should.

This algorithm operates on pixels instead of blocks.

But, before applying the algorithm, a few steps are needed.

1. Process the characters we want to use
2. Preprocess the image (if necessary), as it is weird that an image fits perfectly the first time.

You can jump directly to the [explanation of the algorithm](#3-convert-blocks-to-character) if you want, but to understand how this app works you will need the first two steps.

## 1. Characters

After this step we will have the dimensions of the blocks and a bitmap of each character (including the brightness of its pixels).

1. **Dimensions of the block**

    We assume a monospaced font.

    The dimensions of the blocks need to be calculated because different fonts have different proportions and different gaps between lines.

    Also, the width changes depending on the height of the character.

    The default font used here is `UbuntuMono Nerd Font` with a default height of 16px, which results in 8x16 characters with no line gap.

    The dimensions of the block are (char_width)x(char_height + line_gap)

2. **Character bitmap**

    We iterate over every pixel and calculate its brightness.

    When calculating brightness, a value from 0 to 1 is obtained. It is important to subtract 0.5, because the algorithm only works with positive and negative values. This will make sense later.

    We also collect statistical information on each character to work with different algorithms.

## 2. Preprocessing (if any)

After this step we will have the final image to be converted.

These steps are only executed if the flags say so.

1. **Resize the image**:

    If the dimension has been given in characters, calculate its equivalent in pixels.

    Little explanation here, just resize the image.

2. **Center the image**:

    Sometimes the image doesn't fully fill the right or bottom blocks. In this step we add enough padding so the image does fill them.

3. **Add padding**:

    Add more padding to the image on top of the previous one, if there is any. It is drawn as transparent.

4. **Saturate the image**:

    Increases the color saturation of each pixel if it is brighter than the midpoint brightness and decreases it if it isn't.

5. **Draw borders**:

    You can detect borders by hue, brightness, alpha (transparencies) or a mix of them.

    A border is detected by comparing a pixel with its right and bottom neighbor. If its difference is higher than a threshold, it records that position.

    When all the borders have been identified, there is a second pass on the image that draws them. It draws a square with a specific thickness on each position.

    One las thing we have to take into consideration: the transparent pixels also have a hue, and a brightness. This is because the alpha is a separate channel. A transparent pixel is usually black, but it can have an arbitrary color. This means that it can interact in unexpected ways when detecting borders by hue or brightness.

    To stop transparencies with interfering with hue or brightness, we multiply the difference between them by the alpha of the two pixels (the alpha should be between 0 and 1). This way the more transparent they are the less difference between them.

6. **Treat transparent pixels** (always applied):

    The algorithm doesn't work with alpha values, it can only see brightness.

    Transparencies usually mean that a pixel is less visible, so a pixel is mixed with black (or white, if the arguments say so) in proportion to its alpha.

7. **Apply negative effect**:

    Inverts the brightness of the image keeping the hue.

    It takes into consideration how brightness is measured (see [calculating brightness](#calculating-brightness)), so you actually see the inverted image.

8. **Convert to grayscale**:

    It consists of two steps.

    - Convert the colors to their equivalent in grayscale.
    - Normalize the brightness: The min is 0 and the max is 1.

9. **Convert to black and white**:

    For each pixel, if its brightness is over a threshold, make it white. Otherwise, make it black.

## 3. Convert Blocks to Character

After this step we will have a string with the characters that match the image the best.

First, the image is divided into blocks, and then each block is compared to each character.

After processing a block, we should have the character that fits it best.

1. **Preprocessing a block**

    We have a bitmap of each character, and a bitmap of the block.

    We get the bitmap of the block in the same way as the characters. We calculate the brightness of each pixel and then subtract 0.5 (this value can be changed in this app).

2. **Finding the best match**

    For each character, each pixel is multiplied by its equivalent in the block, and all values are added (`[0][0] * [0][0] + [0][1] * [0][1] + ... + [n][m] * [n][m]`).

    This works because it rewards matching pixels (both with positive or negative brightness) and punishes mismatching ones (one positive and the other negative).

    The character with the highest score is the one that matches the best.

    **Optimization:**

    It only applies with this algorithm when the first character is a space (I think there can be a general solution, but I do not have it yet).

    In this step, the number of bright pixels in the block is also counted. With bright I mean with a brightness higher than the midpoint brightness, as we subtract the midpoint brightness to the original brightness.

    The base case is simple. If it doesn't have bright pixels, then the best match is always the space. It doesn't matter that there are other characters with a similar shape, because when multiplying the space has the greatest values, so it will have the highest result.

    Now comes a complicated phrase, but in the following paragraph we give an example. This is the main logic for the optimization:

    A character is only considered for printing if half of its bright pixels are at least the number of illuminated pixels in the block.

    The sentence is complicated to understand, but in summary, if a character has 10 bright pixels, a character will only be considered if it has at least 5 bright pixels. If it had less than 5, there would be no scenario where that character would be chosen before the space. If there is no space, the character with the fewest illuminated pixels is chosen.

    Additionally, if all pixels are completely illuminated, the brightest character can be printed directly. Notice that if they were not completely illuminated there might be combinations where other character fits best.

## Other algorithms

This app has other algorithms that can match a block:

The first one is called `max_prod` and it is the one we have explained before. None of the others can match the results of this one except `ncc`, which acts in a similar way, so the results are practically the same.

- **Maximum Product (`max_prod`)**: The default algorithm described in step 3.2. Multiplies each pixel of the character with its equivalent in the block and sums all values (`block[0] * char[0] + block[1] * char[1] + ...`). The character with the highest score wins. This rewards matching pixels (both bright or both dark) and punishes mismatches.

- **Minimum Difference (`min_diff`)**: Calculates the absolute difference between each pixel of the character and the block, then sums them (`|block[0] - char[0]| + |block[1] - char[1]| + ...`). The character with the lowest total difference is selected.

- **Minimum Squared Difference (`min_diff_sq`)**: Like `min_diff`, but squares each difference before summing (`(block[0] - char[0])² + (block[1] - char[1])² + ...`). This penalizes larger differences more heavily. The character with the lowest total is selected.

- **Correlation (`correlation`)**: Calculates the Pearson correlation coefficient between the block and each character. This algorithm focuses on pattern structure, not brightness level. It measures how similar the shape is, regardless of how bright or dark the overall block is. Returns the character with the highest correlation.

- **Normalized Cross-Correlation (`ncc`)**: Similar to correlation, but takes brightness level into account. It normalizes the dot product by the magnitudes of both vectors (`Σ(block * char) / (||block|| * ||char||)`). This measures both pattern similarity and brightness matching. The character with the highest NCC value is selected.

- **Gradient (`gradient`)**: Takes into account only the average brightness of the block, ignoring pixel patterns. It calculates the mean brightness of the block and compares it with the average brightness of each character. The character with the closest average brightness wins.

## Calculating brightness

### Brightness formula

In the previous sections, when calculating the brightness from a rgb value, we have used the following formula:

sqrt(0.299 \* r + 0.587 \* g + 0.114 \* b)

We do the square root because the human eye does not perceive the brightness linearly. If we didn't do the square root, the color red would be dark enough that it would not print.

### Custom brightness formula

It is the same as before but you subtract 0.5 (or other value of your liking). The point is that you need negative and positive values.

If the brightness range is from 0 to 1, when you multiply you can only increase the score, so a brighter character will have a higher value than the darker ones. Even if it was a completely dark block it would have a score of 0 with every character.

It is only when you have negative values that you can punish mismatches.

## Printing color

When printing color we have to take special measures, we can't just take the average of the block.

In older versions this was the case, and in photos with borders you usually had color leaking on some characters, from one side of the border to the other.

This happens because one character spans over the two areas, mixing both colors.

We can fix this by only taking into consideration the color of the pixels where the character and the block are bright.

This way only the color of the pixels that took the decision is printed.
