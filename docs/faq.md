# Frequently Asked Questions

- **How to print a black logo?**
    It's only a problem when the background is transparent. In that case, just add `-n` to the command to print the image in negative. We remember that transparent pixels are never printed.

- **Why does the text print with the same font when I change the font?**
    The application only uses the font to compare each block of the image with the characters. You will need to change the font of the console (or wherever you want to put it) to match. It is likely to be deformed, since the application assumes it is a monospace font with 1x2 proportions.

- **It doesn't read some of the arguments**
    In some cases, when adding characters (at least in Windows 10 powershell), if it ends in `\"` (to add the `"` character), what comes after will be taken as characters to add. The solution is to put the arguments before the character argument.