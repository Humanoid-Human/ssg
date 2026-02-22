use std::{path::PathBuf, fs};

mod walker;
mod process;
mod config;

fn main() {
    let mut args = std::env::args();
    args.next();
    match args.next().expect("No command given").as_ref() {
        "init" => {
            let _ = fs::File::create("ssg.toml");
            let _ = fs::create_dir("include");
            let _ = fs::create_dir("src");
            let _ = fs::create_dir("site");
            let _ = fs::File::create("src/index.md");
        },
        "build" => {
            let mut config = config::Config::default();
            config.update(PathBuf::from("ssg.toml"));

            walker::walk_dir(PathBuf::from(&config.src_path),
                PathBuf::from(&config.dest_path),
                &config);
        },
        _ => {
            panic!("Unrecognized command");
        }
    }   
}
