use crate::config::Config;
use regex::{Captures, Regex};
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

            if process
                && let Some(e) = src_path.extension()
                && e != "html"
            {
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
    let mut header = Some(config.header_path());
    let mut title = config.default_title.as_ref();

    let mut startline = 0;
    for (num, line) in src.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        let mut i = line.splitn(2, ": ");
        match i.next().unwrap() {
            "title" => title = i.next().unwrap_or(&config.default_title),
            "header" => {
                if let Some(b) = i.next() {
                    if b == "none" {
                        header = None;
                    } else {
                        header = Some(PathBuf::from(b));
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

    if startline == 0 {
        header = Some(config.header_path());
        title = &config.default_title;
    }

    let mut parse = "<!DOCTYPE html>\n<html>\n".to_string();

    if let Some(hpath) = header
        && hpath.exists()
    {
        parse.push_str(&include_file(&hpath, default_replace(title)));
        parse.push('\n');
    }

    parse.push_str("<body>\n");

    let mut lines = src.lines();
    if startline != 0 {
        lines.nth(startline).unwrap();
    }

    let incl_re = Regex::new(r"\[\[include (.*?)(?:\s*\|\s*(.*?))?\]\]").unwrap();
    for line in lines {
        parse.push('\n');
        let replace = |c: &Captures| {
            let path = config
                .base_dir
                .join(&config.include_path)
                .join(&c[1]);

            if !path.exists() {
                return c[0].to_string();
            }

            let mut replace_map = default_replace(title);
            if let Some(opts) = &c.get(2) {
                for pair in opts.as_str().split("|") {
                    let mut kv = pair.splitn(2, "=");
                    let key = kv.next().unwrap().trim();
                    if let Some(val) = kv.next() {
                        replace_map.push((key.to_string(), val.trim().to_string()));
                    }
                }
            }
            include_file(&path, replace_map)
        };
        let line = incl_re.replace_all(line, replace);
        parse.push_str(&line);
    }

    parse.push_str("\n</body>\n</html>");

    let mut options = markdown::Options::gfm();
    options.compile.allow_dangerous_html = true;
    options.compile.gfm_tagfilter = false;

    let html = markdown::to_html_with_options(&parse, &options).unwrap();
    dest.write_all(&html.into_bytes())
        .expect("Error: failed to write to file");
}

fn include_file(path: &Path, replace_map: Vec<(String, String)>) -> String {
    assert!(path.exists(), "{:?} does not exist", path);
    let mut s = read_to_string(path).expect("Error: failed to read included file");
    for (key, val) in replace_map {
        s = s.replace(&format!("+{}+", key), &val);
    }

    s
}

fn default_replace(title: &str) -> Vec<(String, String)> {
    vec![("title".to_string(), title.to_string())]
}
