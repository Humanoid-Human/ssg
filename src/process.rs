use crate::config::Config;
use regex_lite::{Captures, Regex};
use std::{
    fs::{self, File, read_to_string},
    io::Write,
    path::{Path, PathBuf},
};

pub fn run(config: &Config) {
    let dest = config.abs_site();
    if dest.exists() {
        fs::remove_dir_all(&dest).unwrap();
    }
    fs::create_dir(&dest).unwrap();

    walk_dir(&config.abs_static(), &dest, false, config);
    walk_dir(&config.abs_src(), &dest, true, config);
}

fn walk_dir(from: &PathBuf, to: &Path, process: bool, config: &Config) {
    for entry in fs::read_dir(from).unwrap() {
        let src_path = entry.unwrap().path();
        let mut dest_path = to.join(src_path.file_name().unwrap());
        if src_path.is_dir() {
            if !dest_path.exists() || !dest_path.is_dir() {
                fs::create_dir(&dest_path).unwrap();
            }
            walk_dir(&src_path, &dest_path, process, config);
        } else {
            if let Ok(rel_path) = src_path.strip_prefix(&config.base_dir) {
                println!("{}", rel_path.to_str().unwrap_or("unprintable path!"));
            } else {
                println!("{}", src_path.to_str().unwrap_or("unprintable path!"));
            }

            let ext = src_path.extension();
            if process && (ext.is_none() || ext.unwrap() != "html") {
                dest_path.set_extension("html");
                let dest_file = File::create(dest_path).unwrap();
                process_file(read_to_string(&src_path).unwrap(), dest_file, config);
            } else {
                #[cfg(target_family = "unix")]
                let symlink = std::os::unix::fs::symlink;
                #[cfg(target_family = "windows")]
                let symlink = std::os::windows::fs::symlink_file;

                symlink(&src_path, dest_path).unwrap();
            }
        }
    }
}

fn process_file(src: String, mut dest: File, config: &Config) {
    let mut head = Some(config.header_path());
    let mut page_start = Some(config.page_start_path());
    let mut page_end = Some(config.page_end_path());
    let mut title = config.default_title.as_ref();

    let mut startline = 0;
    for (num, line) in src.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        let mut i = line.splitn(2, ": ");
        match i.next().unwrap() {
            "title" => title = i.next().unwrap_or(&config.default_title),
            "head" => {
                if let Some(p) = i.next() {
                    if p == "none" {
                        head = None;
                    } else {
                        head = Some(config.abs_include().join(PathBuf::from(p)));
                    }
                }
            }
            "page_start" => {
                if let Some(p) = i.next() {
                    if p == "none" {
                        page_start = None;
                    } else {
                        page_start = Some(config.abs_include().join(PathBuf::from(p)));
                    }
                }
            }
            "page_end" => {
                if let Some(p) = i.next() {
                    if p == "none" {
                        page_end = None;
                    } else {
                        page_end = Some(config.abs_include().join(PathBuf::from(p)));
                    }
                }
            }
            "++++" => {
                startline = num;
                break;
            }
            _ => (),
        }
    }

    let title_replace = Some(vec![("title", title)]);

    let mut header = "<!DOCTYPE html>\n<html>\n".to_string();

    if let Some(hpath) = head
        && hpath.exists()
    {
        header.push_str(&include_file(&hpath, &title_replace));
        header.push('\n');
    }

    header.push_str("<body>\n");

    let mut parse = String::new();

    let mut lines = src.lines();
    if startline != 0 {
        lines.nth(startline).unwrap();
    }

    let incl_re = Regex::new(r"\{\{i (.*?)(?:\s*\|\s*(.*?))?}}").unwrap();
    for line in lines {
        parse.push('\n');
        let replace = |c: &Captures| {
            let mut path = config.base_dir.join(&config.include_path).join(&c[1]);
            if path.extension().is_none() {
                if !path.exists() {
                    path.set_extension("html");
                }
                if !path.exists() {
                    path.set_extension("md");
                }
            }

            if !path.exists() {
                eprintln!("Warning: {}{} not found", &config.include_path, &c[1]);
                return c[0].to_string();
            }

            let mut replace_map = Vec::new();
            if let Some(opts) = &c.get(2) {
                for pair in opts.as_str().split("|") {
                    let mut kv = pair.splitn(2, "=");
                    let key = kv.next().unwrap().trim();
                    if let Some(val) = kv.next() {
                        replace_map.push((key, val.trim()));
                    }
                }
            }
            include_file(&path, &Some(replace_map))
        };
        let line = incl_re.replace_all(line, replace);
        parse.push_str(&line);
    }

    parse = parse.replace("{title}", title);

    let mut options = markdown::Options::gfm();
    options.compile.allow_dangerous_html = true;
    options.compile.gfm_tagfilter = false;

    let html = markdown::to_html_with_options(&parse, &options).unwrap();

    let write_err = "Error: failed to write to file";
    dest.write_all(&header.into_bytes()).expect(write_err);
    if let Some(start) = page_start {
        dest.write_all(&include_file(&start, &title_replace).into_bytes()).expect(write_err);
    }
    dest.write_all(&html.into_bytes()).expect(write_err);
    if let Some(end) = page_end {
        dest.write_all(&include_file(&end, &title_replace).into_bytes()).expect(write_err);
    }
    dest.write_all(b"</body></html>").expect(write_err);
}

fn include_file(path: &Path, replace_map: &Option<Vec<(&str, &str)>>) -> String {
    let mut s = read_to_string(path).expect("Error: failed to read included file");
    if let Some(replace_map) = replace_map {
        for (key, val) in replace_map {
            s = s.replace(&format!("{{{}}}", key), val);
        }
    }

    s
}
