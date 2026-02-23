use std::{path::PathBuf, fs::read_to_string};

pub struct Config {
    pub src_path:      String,
    pub dest_path:     String,
    pub default_title: String,
    pub default_date:  String,
    pub include_path:  String,
    pub header_name:   String,
    pub footer_name:   String
}

impl Config {
    pub fn default() -> Self {
        Self {
            src_path:      "src/".to_string(),
            dest_path:     "site/".to_string(),
            default_title: "Page Title".to_string(),
            default_date:  "0000-00-00".to_string(),
            include_path:  "include/".to_string(),
            header_name:   "header.html".to_string(),
            footer_name:   "footer.html".to_string()
        }
    }

    pub fn update(&mut self, path: PathBuf) {
        if !path.exists() { return; }
        for line in read_to_string(path).unwrap().lines() {
            let mut split = line.splitn(2, "=");
            let thing = match split.next().unwrap().trim() {
                "src_path" => &mut self.src_path,
                "dest_path" => &mut self.dest_path,
                "default_title" => &mut self.default_title,
                "default_date" => &mut self.default_title,
                "include_path" => &mut self.include_path,
                "header_name" => &mut self.header_name,
                "footer_name" => &mut self.footer_name,
                _ => continue
            };
            *thing = split.next().unwrap().trim().to_string();
            if let Some(s) = thing.strip_prefix("\"") {
                *thing = s.to_string();
            }
            if let Some(s) = thing.strip_suffix("\"") {
                *thing = s.to_string();
            }
        }
    } 

    pub fn header_path(&self) -> String {
        format!("{}{}", self.include_path, self.header_name)
    }

    pub fn footer_path(&self) -> String {
        format!("{}{}", self.include_path, self.footer_name)
    }
}
