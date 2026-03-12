use std::{path::PathBuf, fs, env::current_dir};

mod process;
mod config;
mod server;

fn main() {
    let mut args = std::env::args();
    args.next();
    match args.next().expect("No command given").as_ref() {
        "init" => {
            config::gen_default_file(PathBuf::from("ssg.conf"));
            fs::create_dir("include").unwrap();
            fs::create_dir("src").unwrap();
            fs::create_dir("_site").unwrap();
            fs::create_dir("static").unwrap();
            fs::File::create_new("include/head.html").unwrap();
        },
        "build" => {
            let base = find_base_dir();
            process::run(&config::Config::new(base.join("ssg.conf")));
        },
        "server" => {
            let base = find_base_dir();
            let config = config::Config::new(base.join("ssg.conf"));
            process::run(&config);
            println!("Running http server at http://127.0.0.1:{}", config.server_port);
            server::run_server(&config.server_port, config.abs_site());
        }
        _ => {
            panic!("Unrecognized command");
        }
    }   
}

fn find_base_dir() -> PathBuf {
    let mut curdir = current_dir().unwrap();
    loop {
        if curdir.join("ssg.conf").exists() {
            return curdir;
        }

        match curdir.parent() {
            Some(parent) => curdir = parent.to_path_buf(),
            None => panic!("Could not find `ssg.conf` in `{}` or any parent directory", current_dir().unwrap().display())
        }
    }
}