use std::{fs, io::{BufReader, BufRead}, path::PathBuf};

pub fn build(drafts: bool) {
    let input = PathBuf::from("_src");
    build_dir(drafts, &input);
}

fn build_dir(drafts: bool, input: &PathBuf) {
    let items = fs::read_dir(input).expect("could not open directory");
    for item in items {
        let item = item.expect("could not read file in directory");
        let path = item.path();
        let out_path = to_output(&path);
        if item.file_type().unwrap().is_dir() {
            build_dir(drafts, &path);
        } else {
            build_file(drafts, &path, &out_path);
        }
    }
}

fn build_file(drafts: bool, path: &PathBuf, out_path: &PathBuf) {
    let f = fs::File::open(path).unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = String::new();
    if reader.read_line(&mut buf).unwrap() == 4 && buf == "%%%\n" {
        // handle header
    }
    // do other stuff
}

fn to_output(path: &PathBuf) -> PathBuf {
    let s = path.to_str().unwrap().replacen("_src", "_site", 1);
    PathBuf::from(s)
}