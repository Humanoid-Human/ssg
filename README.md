# ssg
Simple static site generator.

By default, the source (Markdown) goes in `src`, and the output goes in `_site`.
These paths can be configured.

ssg uses the GFM extension to CommonMark.

## Usage
`ssg init`: Initializes the current directory. This command generates the `_site`, `src`, and `include` directories, as well as `ssg.conf`.

`ssg build`: Builds all source files. This command will remove everything in `_site`, and rebuild using the contents of `src`.

`ssg server`: Runs `ssg build` and then starts a localhost server for previewing the site, on port 8000 by default.

## Directory Structure
- `src`: Stuff that should be processed by the tool, typically MarkDown files. The only exception is that files ending in `.html` will not be processed.
- `static`: Stuff that should be included in the site but not processed, such as images, css files, etc.
- `include`: Stuff that is included into files in `src`.
- `_site`: The generated site. Do not edit files in this directory, as it is removed and re-created when the `ssg build` or `ssg server` is run.
- `ssg.conf`: Configuration file.

## Configuration
Configure stuff in `ssg.conf`. There exist the following options:
- `src_path`
- `static_path`
- `include_path`
- `site_path`
- `header_name`
- `default_title`
- `default_date`
- `server_port`
Modify them with the syntax `option: value`.

## Features
The first section of page, before the `++++`, is the frontmatter. May contain the following info:
- `title` (`string`): Title of the page (default `Page Title`). Ex: `title: Different Page Title`
- `date` (`string`): Date of the page (default `0000-00-00`). Ex: `date: 1969-12-31`
- `head` (`string`): Path to an HTML file to use as the header, instead of the default, relative to `include/`. Set this to `none` to not include a header.
- some other stuff probably

### Includes
`[[include filename]]` attempts to copy the contents of `include/filename` into the file by default.
This path can be changed in the configuration.
Recursive includes are not supported; only includes present in the original file will be processed.

Additionally, by default the file `include/head.html` is included into the beginning of the file.
To disable this, add `head: none` to the frontmatter.

In any of the above file inclusions, `+title+` will be replaced with the title of the page, and `+date+` will be replaced with the date of the page.

Additional replacements can be defined in the include statement; for example,
`[[include page | foo = bar | baz = boo ]]`
will include the file `include/page`, with every instance of `+foo+` replaced with `bar`, and every instance of `+baz+` replaced with `boo`.

## Libraries
This project depends on the following libraries:
- [`markdown`](https://crates.io/crates/markdown)
- [`regex`](https://crates.io/crates/regex)

## Building
Clone the repo and build with `cargo`.
