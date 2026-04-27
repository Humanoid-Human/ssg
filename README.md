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
- `page_start_name`
- `page_end_name`
- `default_title`
- `server_port`

Modify them with the syntax `option: value`.

## Features
The first section of a file in `src`, separated from the main content by `++++`, is the frontmatter.
It may contain the following variables, set using `key: value` syntax:

- `title`: Title of the page.
- `head`: Path to an HTML file to use as the header, instead of the default, relative to `include/`. Set to `none` to disable header.
- `page_start`: Path to a page-start file to use instead of the default. Set to `none` to disable.
- `page_end`: Path to a page-end file to use instead of the default. Set to `none` to disable.

All of the above variables are optional.

### Includes
`{{i filename}}` attempts to copy the contents of `include/filename` into the file during processing.
If this file does not exist, and `filename` does not have a file extension, then the file extensions `.html` and `.md` will be checked.
The include path can be changed in the configuration.
Recursive includes are not supported; only includes present in the original file will be processed.

Replacements can be defined in the include statement; for example,
`{{i page | foo = bar | baz = boo }}`
will include the file `include/page`, with every instance of `{foo}` replaced with `bar`,
and every instance of `{baz}` replaced with `boo`.

### Header & Footer
If it exists, `include/head.html` is included into the beginning of the file, after `<!DOCTYPE html><html>`.
This is intended for use with the HTML `<head>` tag.
The default path can be modified in `ssg.conf`, and the header used for a specific page can be modified in the frontmatter.
To disable, set

### Page Start & End
If it exists, `include/page_start.html` is included into the start of the body,
and `include/page_end.html` is included into the end of the body.
The default paths can be modified in `ssg.conf`, and the page start/end used for a specific page can be modified in the frontmatter.

### Title replacement
The string `{title}` will be replaced with the title of the page, wherever it appears, after all other processing.

## Dependencies
`ssg` uses the following libraries:
- [`markdown`](https://crates.io/crates/markdown)
- [`regex-lite`](https://crates.io/crates/regex-lite)

## Building
Clone the repo and build with `cargo`.
