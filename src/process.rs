use std::{
    io::Write,
    fs, fs::{File, read_to_string},
    path::{Path, PathBuf}
};
use regex::{Captures, Regex};
use crate::config::Config;

pub fn walk_dir(src: PathBuf, dest: PathBuf, config: &Config) {
    if !src.is_dir() { return; }

    fs::remove_dir_all(&dest).unwrap();
    fs::create_dir(&dest).unwrap();

    for entry in fs::read_dir(&src).unwrap() {
        let src_path = entry.unwrap().path();
        let mut dest_path = dest.join(src_path.file_name().unwrap());
        if src_path.is_dir() {
            fs::create_dir(&dest_path).unwrap();
            walk_dir(src_path, dest_path, config);
        } else if dest_path.extension().is_some_and(|x| x == "md") {
            dest_path.set_extension("html");
            let dest_file = File::create(dest_path).unwrap();
            process_file(read_to_string(src_path).unwrap(), dest_file, config);
        } else {
            fs::copy(src_path, dest_path).unwrap();
        }
    }
}

fn process_file(src: String, mut dest: File, config: &Config) {
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
        let replace = |c: &Captures| {
            let path = config.base_dir.join(&format!("{}{}", config.include_path, &c[1]));
            if path.exists() {
                return include_file(&path, title, date);
            } else {
                return format!("[[include {}]]", c[1].to_string());
            }
        };  
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
    assert!(path.exists());
    read_to_string(path).unwrap()
        .replace("+title+", page_title)
        .replace("+date+", page_date)
}
