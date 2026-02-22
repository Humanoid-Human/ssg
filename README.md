# ssg
Static site generator in rust.

Source (Markdown) goes in `src`, output goes in `site`. This system conforms to CommonMark markdown.

## Features:
The first section of page, before the `++++`, is the header. May contain the following info, in TOML format:
- `title` (`string`): Title of the page (mandatory)
- `head`: Whether or not to include the common header (default `true`)
- `foot` (`bool`): Whether or not to include the common footer (default `true`)
- some other stuff probably

`[[include filename]]` attempts to copy the contents of `.ssg/include/filename` into the file.

If the relevant options are enabled, the contents of `.ssg/head.html` and `.ssg/foot.html` are copied into the beginning and end of the file, respectively.

