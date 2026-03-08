# ssg
Simple static site generator.

By default, the source (Markdown) goes in `src`, and the output goes in `site`. These paths can be configured.

ssg uses the GFM extension to CommonMark.

## Usage
`ssg init`: Initializes the current directory. This command generates the `site`, `src`, and `include` directories, as well as `ssg.conf`.
`ssg build`: Builds all source files. This command will remove everything in `site`, and rebuild using the contents of `src`.
`ssg run`: Runs `ssg build` and then starts a localhost server for previewing the site, on port 8000 by default

## Configuration
Configure stuff in `ssg.conf`. There exist the following options:
- `src_path`
- `dest_path`
- `default_title`
- `default_date`
- `include_path`
- `header_name`
- `footer_name`
- `localhost_port`
Modify them with the syntax `option = value`.

## Features
The first section of page, before the `++++`, is the frontmatter. May contain the following info:
- `title` (`string`): Title of the page (default `Page Title`). Ex: `title: Different Page Title`
- `date` (`string`): Date of the page (default `0000-00-00`). Ex: `date: 1969-12-31`
- `head` (`string`): Path to an HTML file to use as the header, instead of the default. Set this to `none` to not include a header.
- some other stuff probably

`[[include filename]]` attempts to copy the contents of `include/filename` into the file by default.
This path can be changed in the configuration.
Recursive includes are not supported; only includes present in the original file will be processed.

If the relevant options are enabled, the contents of `include/head.html` are copied into the beginning of the file. This path can be changed in the configuration. Note that this feature is meant for the HTML `<head>` element. The remainder of the page content is automatically wrapped in `<body>`. 

In any of the above file inclusions, `+title+` will be replaced with the title of the page, and `+date+` will be replaced with the date of the page.

## Libraries
This project uses:
- [`markdown`](https://crates.io/crates/markdown) (sometimes called `markdown-rs`) for Markdown parsing.
- [`regex`](https://crates.io/crates/regex) for regex (wow!)

## Building
Clone the repo and build with `cargo`.
