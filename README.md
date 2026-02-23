# ssg
Static site generator in rust.

By default, the source (Markdown) goes in `src`, and the output goes in `site`. These paths can be configured.

ssg uses the GFM extension to CommonMark.

## Usage
`ssg init`: Initializes the current directory
`ssg build`: Builds all source files

## Configuration
Configure stuff in `ssg.toml`. There exist the following options:
- `src_path`
- `dest_path`
- `default_title`
- `default_date`
- `include_path`
- `header_name`
- `footer_name`

## Features
The first section of page, before the `++++`, is the header. May contain the following info:
- `title` (`string`): Title of the page (default `Page Title`). Ex: `title: Different Page Title`
- `date` (`string`): Date of the page (default `0000-00-00`). Ex: `date: 1969-12-31`
- `header` (`bool`): Whether or not to include the common header (default `true`). Ex: `header: false`
- `footer` (`bool`): Whether or not to include the common footer (default `true`). Ex: `footer: false`
- some other stuff probably

`[[include filename]]` attempts to copy the contents of `include/filename` into the file by default. This path can be changed in the configuration.

If the relevant options are enabled, the contents of `include/head.html` and `include/foot.html` are copied into the beginning and end of the file, respectively. These paths can be changed in the configuration.

In any of the above file inclusions, `+title+` will be replaced with the title of the page, and `+date+` will be replaced with the date of the page.

## Building
Install rustc and cargo. Clone the repo. Navigate to the folder and use `cargo build --release`.
