# Use the results

The app currently supports images and gifs.

To save the results you can redirect the output of the terminal into a file with `>` or copy them from the console.

## Images

When processing an image, it prints the ASCII art directly, so you can just paste it wherever you want.

Check out the supported [output formats](./tutorial/tutorial-format.md), so you can choose the one that best fits your needs (terminal app vs the web).

## Gifs

When processing a gif, the app outputs a JSON object with the following format:

```json
{
  "font": "Ubuntu Mono", // Font used when processing the gif
  "width": 62, // Width of the gif in characters
  "height": 31, // Height of the gif in characters
  "frames": [ // List of the frames. A frame contains the ASCII art and the delay of the frame
    {
      "ascii": "<ascii-art-frame0>",
      "delay_ms": 80
    },
      {
      "ascii": "<ascii-art-frame1>",
      "delay_ms": 80
    },
  ]
}
```

The ASCII art of each frame is the same as if you had processed it alone.

Currently there is no way to display the result in the terminal, but there is a [web component](../web/src/lib/web_components/ascii_gif_renderer.ts) that can render the results in the web.
