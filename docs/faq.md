# Frequently Asked Questions

- **How to print a black logo?**
    It's only a problem when the background is transparent. You should add `-nv` to make the transparent pixels white and print the negative version.

- **Why does the text print with the same font when I change the font?**
    The application only uses the font to compare each block of the image with the characters. You will need to change the font of the console (or wherever you want to put it) to match. It is likely to be deformed, since the application assumes it is a monospace.

- **It doesn't read some of the arguments**
    In some cases, when adding characters (at least in Windows 10 powershell), if it ends in `\"` (to add the `"` character), what comes after will be taken as characters to add. The solution is to put the arguments before the character argument.

- **Can we mix multiple character sets (dicts)?**
    Yes, you can use multiple separated by a comma: `-d box_all,blocks`.