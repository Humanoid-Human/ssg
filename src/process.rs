use std::{io::Write, fs::{File, read_to_string}, path::Path};
use regex::{Captures, Regex};
use crate::config::Config;

pub fn file(src: String, mut dest: File, config: &Config) {
    let mut do_header = true;
    let mut do_footer = true;
    let mut title = config.default_title.as_ref();
    let mut date = config.default_date.as_ref();

    let mut startline = 0;
    for (num, line) in src.lines().enumerate() {
        let mut i = line.splitn(2, ": ");
        match i.next().unwrap() {
            "title" => title = i.next().unwrap_or(&config.default_title),
            "date" => date = i.next().unwrap_or(&config.default_date),
            "header" => if let Some(b) = i.next() && b == "false" { do_header = false; },
            "footer" => if let Some(b) = i.next() && b == "false" { do_footer = false; },
            "++++" => { startline = num + 1; break; },
            _ => ()
        }
    }

    if startline == 0 {
        do_header = true;
        do_footer = true;
        title = &config.default_title;
        date = &config.default_date;
    }

    let mut parse = String::new();

    if do_header {
        let hpath = config.header_path();
        let hpath = Path::new(&hpath);
        if hpath.exists() {
            parse = read_to_string(hpath).unwrap()
                .replace("+title+", title)
                .replace("+date+", date);
            parse.push('\n');
        }
    }
    
    let mut lines = src.lines();
    parse.push_str(lines.nth(startline).unwrap_or(""));

    let incl_re = Regex::new(r"\[\[include (.*?)\]\]").unwrap();
    for line in lines {
        parse.push('\n');
        let replace = |c: &Captures|
            include_file(Path::new(&format!("{}{}", config.include_path, &c[1])), title, date);
        let line = incl_re.replace_all(line, replace);
        parse.push_str(&line);
    }

    if do_footer {
        parse.push('\n');
        parse.push('\n');
        let fpath = config.footer_path();
        let fpath = Path::new(&fpath);
        if fpath.exists() {
            parse.push_str(&read_to_string(fpath).unwrap()
                .replace("+title+", title)
                .replace("+date+", date));
        }
    }
    
    let html = markdown::to_html_with_options(&parse, &markdown::Options::gfm()).unwrap();
    dest.write_all(&html.into_bytes()).unwrap();
}

fn include_file(path: &Path, page_title: &str, page_date: &str) -> String {
    if !path.exists() { return String::new(); }
    
    read_to_string(path).unwrap()
        .replace("+title+", page_title)
        .replace("+date+", page_date)
}