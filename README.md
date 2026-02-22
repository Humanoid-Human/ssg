# ssg
Static site generator in rust.

Source (Markdown) goes in `src`, output goes in `site`. This system conforms to CommonMark markdown.

## Features:
The first section of page, before the `++++`, is the header. May contain the following info:
- `title` (`string`): Title of the page (default `Page Title`). Ex: `title: Different Page Title`
- `date` (`string`): Date of the page (default `<date>`). Ex: `date: 1969-12-31`
- `header` (`bool`): Whether or not to include the common header (default `true`). Ex: `header: false`
- `footer` (`bool`): Whether or not to include the common footer (default `true`). Ex: `footer: false`
- some other stuff probably

`[[include filename]]` attempts to copy the contents of `.ssg/include/filename` into the file.

If the relevant options are enabled, the contents of `.ssg/head.html` and `.ssg/foot.html` are copied into the beginning and end of the file, respectively.

In any of the above file inclusions, `+title+` will be replaced with the title of the page, and `+date+` will be replaced with the date of the page.