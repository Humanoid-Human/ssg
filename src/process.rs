use std::{io::Write, fs::{File, read_to_string}, path::Path};
use regex::{Captures, Regex};
use crate::config::Config;

pub fn file(src: String, mut dest: File, config: &Config) {
    let mut header = Some(config.header_path());
    let mut title = config.default_title.as_ref();
    let mut date = config.default_date.as_ref();

    let mut startline = 0;
    for (num, line) in src.lines().enumerate() {
        let mut i = line.splitn(2, ": ");
        match i.next().unwrap() {
            "title" => title = i.next().unwrap_or(&config.default_title),
            "date" => date = i.next().unwrap_or(&config.default_date),
            "header" => if let Some(b) = i.next() {
                if b == "none" { header = None; }
                else { header = Some(b.to_string()); }
            },
            "++++" => { startline = num; break; },
            _ => ()
        }
    }

    if startline == 0 {
        header = Some(config.header_path());
        title = &config.default_title;
        date = &config.default_date;
    }

    let mut parse = "<!DOCTYPE html>\n<html>\n".to_string();

    if let Some(hpath) = header {
        let hpath = Path::new(&hpath);
        if hpath.exists() {
            parse.push_str(&read_to_string(hpath).unwrap()
                .replace("+title+", title)
                .replace("+date+", date));
            parse.push('\n');
        }
    }

    parse.push_str("<body>\n");
    
    let mut lines = src.lines();
    if startline != 0 {
        lines.nth(startline).unwrap();
    }

    let incl_re = Regex::new(r"\[\[include (.*?)\]\]").unwrap();
    for line in lines {
        parse.push('\n');
        let replace = |c: &Captures|
            include_file(Path::new(&format!("{}{}", config.include_path, &c[1])), title, date);
        let line = incl_re.replace_all(line, replace);
        parse.push_str(&line);
    }

    parse.push_str("\n</body>\n</html>");
   
    let mut options = markdown::Options::gfm();
    options.compile.allow_dangerous_html = true;
    options.compile.gfm_tagfilter = false;

    let html = markdown::to_html_with_options(&parse, &options).unwrap();
    dest.write_all(&html.into_bytes()).unwrap();
}

fn include_file(path: &Path, page_title: &str, page_date: &str) -> String {
    if !path.exists() { return String::new(); }
    
    read_to_string(path).unwrap()
        .replace("+title+", page_title)
        .replace("+date+", page_date)
}
