use std::{
    path::PathBuf,
    fs::{read_to_string, File},
    io::Write
};

pub fn gen_default_file(path: PathBuf) {
    let mut f = File::create_new(path).unwrap();
    f.write_all(b"src_path: src/\n").unwrap();
    f.write_all(b"dest_path: _site/\n").unwrap();
    f.write_all(b"include_path: include/\n").unwrap();
    f.write_all(b"header_name: head.html\n").unwrap();
    f.write_all(b"default_title: Page Title\n").unwrap();
    f.write_all(b"default_date: 0000-00-00\n").unwrap();
    f.write_all(b"server_port: 8000\n").unwrap();
}

pub struct Config {
    pub base_dir:      PathBuf,
    pub src_path:      String,
    pub dest_path:     String,
    pub default_title: String,
    pub default_date:  String,
    pub include_path:  String,
    pub header_name:   String,
    pub server_port:   String
}

impl Config {
    pub fn new(path: PathBuf) -> Self {
        let mut out = Self {
            base_dir: path.parent().unwrap().to_path_buf(),
            src_path: "".to_string(),
            dest_path: "".to_string(),
            include_path: "".to_string(),
            header_name: "".to_string(),
            default_title: "".to_string(),
            default_date: "".to_string(),
            server_port: "".to_string()
        };

        assert!(path.exists());
        for line in read_to_string(path).unwrap().lines() {
            let mut split = line.splitn(2, ":");
            let thing = match split.next().unwrap().trim() {
                "src_path" => &mut out.src_path,
                "dest_path" => &mut out.dest_path,
                "default_title" => &mut out.default_title,
                "default_date" => &mut out.default_title,
                "include_path" => &mut out.include_path,
                "header_name" => &mut out.header_name,
                "server_port" => &mut out.server_port,
                unknown => panic!("Unexpected option {} in ssg.conf", unknown)
            };
            *thing = split.next().unwrap().trim().to_string();
            if let Some(s) = thing.strip_prefix("\"") {
                *thing = s.to_string();
            }
            if let Some(s) = thing.strip_suffix("\"") {
                *thing = s.to_string();
            }
        }

        out
    } 

    pub fn header_path(&self) -> PathBuf {
        self.base_dir.join(&self.include_path).join(&self.header_name)
    }
}
