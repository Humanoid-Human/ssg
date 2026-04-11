# ssg
Simple static site generator.

By default, the source (Markdown) goes in `src`, and the output goes in `_site`.
These paths can be configured.

`ssg` uses the GFM extension to CommonMark.

## Usage
`ssg init`: Initializes the current directory. This command generates the `_site`,
`src`, and `include` directories, as well as `ssg.conf`.

`ssg build`: Builds all source files. This command will remove everything in `_site`,
and rebuild using the contents of `src` and `static`.

`ssg server`: Runs `ssg build` and then starts a localhost server, on port 8000 by default.

## Directory Structure
- `src/`: Files that should be processed by the tool, typically MarkDown. Files ending in `.html` will not be processed.
- `static/`: Files that should be included in the site but not processed, such as images, stylesheets, etc. These files will be symlinked directly into `_site/`.
- `include/`: Files for including into files in `src/` (see [includes](#includes)).
- `_site/`: The generated site. Do not edit files in this directory, as they are removed and re-created when the `ssg build` or `ssg server` is run.
- `ssg.conf`: Configuration file.

## Configuration
Configure stuff in `ssg.conf`. There exist the following options:
- `src_path`
- `static_path`
- `include_path`
- `site_path`
- `header_name`
- `footer_name`
- `default_title`
- `default_date`
- `server_port`
Modify them with the syntax `option: value`.

## Features
The first section of page, before the `++++`, is the frontmatter. May contain the following info:
- `title` (`string`): Title of the page (default `Page Title`). Ex: `title: Different Page Title`
- `head` (`string`): Path to an HTML file to use as the header, instead of the default, relative to `include/`. Set this to `none` to not include a header.
- `foot` (`string`): Same as `head`, but for the footer.

### Includes
`{{i filename}}` attempts to copy the contents of `include/filename` into the file during processing.
If this file does not exist, and `filename` does not have a file extension, then the file extensions `.html` and `.md` will be checked.
The include path can be changed in the configuration.
Recursive includes are not supported; only includes present in the original file will be processed.

If it exists, `include/head.html` is included into the beginning of the file.
This is intended for use with the HTML `<head>` tag.
Similarly, if it exists, `include/foot.html` is included into the end of the file.
However, unlike `head.html`, the footer will be enclosed within the body tags.
To disable these, add `head: none` and `foot: none`, respectively, to the frontmatter.

In any of the above file inclusions, `{title}` will be replaced with the title of the page.
Additional replacements can be defined in the include statement; for example,
`{{i page | foo = bar | baz = boo }}`
will include the file `include/page`, with every instance of `{foo}` replaced with `bar`, and every instance of `{baz}` replaced with `boo`.

## Dependencies
`ssg` uses the following libraries:
- [`markdown`](https://crates.io/crates/markdown)
- [`regex-lite`](https://crates.io/crates/regex-lite)

## Building
Clone the repo and build with `cargo`.
