# Tutorial: HTML output

You can specify the output format with `--format`

- `--format ansi`: (default in cli) It is raw text, only affects when printing color.
- `--format html`: (default in the web) Wraps the drawing in a `<pre>` tag and specifies the font-family and font-size used. When printing color wraps every character with a `<span>` tag with their respective color.
