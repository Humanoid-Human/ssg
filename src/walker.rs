use std::{fs, fs::File, path::PathBuf};
use crate::{process, config};

pub fn walk_dir(src: PathBuf, dest: PathBuf, config: &config::Config) {
    if !src.is_dir() { return; }

    fs::remove_dir_all(&dest).unwrap();
    fs::create_dir(&dest).unwrap();

    for entry in fs::read_dir(&src).unwrap() {
        let entry = entry.unwrap();
        let src_path = entry.path();
        let mut dest_path = dest.clone();
        dest_path.push(src_path.file_name().unwrap());
        if src_path.is_dir() {
            fs::create_dir(&dest_path).unwrap();
            walk_dir(src_path, dest_path, config);
        } else if dest_path.extension().is_some_and(|x| x == "md") {
            dest_path.set_extension("html");
            let dest = File::create(dest_path).unwrap();
            process::file(fs::read_to_string(src_path).unwrap(), dest, config);
        } else {
            fs::copy(src_path, dest_path).unwrap();
        }
    }
}