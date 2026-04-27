use std::{
    fs::{File, read_to_string},
    io::Write,
    path::PathBuf,
};

pub fn gen_default_file(path: PathBuf) {
    let f = File::create_new(path);
    if f.is_err() {
        return;
    }
    let mut f = f.unwrap();
    f.write_all(
        b"src_path: src/
static_path: static/
include_path: include/
site_path: _site/
header_name: head.html
page_start_name: page_start.html
page_end_name: page_end.html
default_title: Page Title
server_port: 8000",
    )
    .expect("Error: failed to write to ssg.conf");
}

pub struct Config {
    pub base_dir: PathBuf,
    pub src_path: String,
    pub static_path: String,
    pub site_path: String,
    pub default_title: String,
    pub include_path: String,
    pub header_name: String,
    pub page_start_name: String,
    pub page_end_name: String,
    pub server_port: String,
}

impl Config {
    pub fn new(path: PathBuf) -> Self {
        let mut out = Self {
            base_dir: path.parent().unwrap().to_path_buf(),
            src_path: "".to_string(),
            static_path: "".to_string(),
            site_path: "".to_string(),
            include_path: "".to_string(),
            header_name: "".to_string(),
            page_start_name: "".to_string(),
            page_end_name: "".to_string(),
            default_title: "".to_string(),
            server_port: "".to_string(),
        };

        assert!(path.exists(), "{:?} does not exist", path);
        for line in read_to_string(path)
            .expect("could not open ssg.conf")
            .lines()
        {
            if line.is_empty() {
                continue;
            }
            let mut split = line.splitn(2, ":");
            let key = split.next().unwrap().trim();
            let value = match key {
                "src_path" => &mut out.src_path,
                "static_path" => &mut out.static_path,
                "include_path" => &mut out.include_path,
                "site_path" => &mut out.site_path,
                "default_title" => &mut out.default_title,
                "header_name" => &mut out.header_name,
                "page_start_name" => &mut out.page_start_name,
                "page_end_name" => &mut out.page_end_name,
                "server_port" => &mut out.server_port,
                unknown => {
                    eprintln!("Warning: unknown option {unknown} in ssg.conf");
                    continue;
                }
            };
            if let Some(val) = split.next() {
                *value = val.trim().to_string();
                if let Some(s) = value.strip_prefix("\"") {
                    *value = s.to_string();
                }
                if let Some(s) = value.strip_suffix("\"") {
                    *value = s.to_string();
                }
            } else {
                eprintln!("Warning: empty option {key} in ssg.conf");
            }
        }

        out
    }

    pub fn header_path(&self) -> PathBuf {
        self.base_dir
            .join(&self.include_path)
            .join(&self.header_name)
    }

    pub fn page_start_path(&self) -> PathBuf {
        self.base_dir
            .join(&self.include_path)
            .join(&self.page_start_name)
    }

    pub fn page_end_path(&self) -> PathBuf {
        self.base_dir
            .join(&self.include_path)
            .join(&self.page_end_name)
    }

    pub fn abs_src(&self) -> PathBuf {
        self.base_dir.join(&self.src_path)
    }

    pub fn abs_static(&self) -> PathBuf {
        self.base_dir.join(&self.static_path)
    }

    pub fn abs_include(&self) -> PathBuf {
        self.base_dir.join(&self.include_path)
    }

    pub fn abs_site(&self) -> PathBuf {
        self.base_dir.join(&self.site_path)
    }
}
