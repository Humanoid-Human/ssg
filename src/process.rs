use std::{io::Write, fs::{File, read_to_string}, path::Path};
use regex::{Captures, Regex};
use markdown;

const default_title: &str = "Page Title";
const default_date: &str = "";

pub fn file(src: String, mut dest: File) {
    let mut do_header = true;
    let mut do_footer = true;
    let mut title = default_title;
    let mut date = default_date;

    let mut startline = 0;
    for (num, line) in src.lines().enumerate() {
        let mut i = line.splitn(2, ": ");
        match i.next().unwrap() {
            "title" => title = i.next().unwrap_or(default_title),
            "date" => date = i.next().unwrap_or(default_date),
            "header" => if let Some(b) = i.next() && b == "false" { do_header = false; },
            "footer" => if let Some(b) = i.next() && b == "false" { do_footer = false; },
            "++++" => { startline = num + 1; break; },
            _ => ()
        }
    }

    if startline == 0 {
        do_header = true;
        do_footer = true;
        title = default_title;
        date = default_date;
    }

    let mut parse = String::new();

    if do_header {
        let hpath = Path::new(".ssg/header.html");
        if hpath.exists() {
            parse = read_to_string(hpath).unwrap()
                .replace("+title+", title)
                .replace("+date+", date);
        }
    }
    
    let mut lines = src.lines();
    parse.push_str(lines.nth(startline).unwrap_or(""));

    for line in lines {
        parse.push('\n');

        let incl_re = Regex::new(r"\[\[include (.*?)\]\]").unwrap();
        let line = incl_re.replace_all(line, |c: &Captures| include_file(Path::new(&c[1]), title, date));
        parse.push_str(&line);
    }

    if do_footer {
        let fpath = Path::new(".ssg/footer.html");
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