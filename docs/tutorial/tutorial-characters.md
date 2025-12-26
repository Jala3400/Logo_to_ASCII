# Tutorial: Characters

The default character set is `8dbqp '·.` (including the space).

-   To change the character set, use the argument `--chars <characters>`. The character set must be enclosed in `"` or `'` if you want to include the space.
-   To add characters to the default group, use `-a <characters_to_add>`. For example, `-a "_/\\"` will add the characters `_`, `/`, and `\`.
-   To use all printable ASCII characters, add `--all`.
-   To remove characters, use `-x <characters_to_remove>`.