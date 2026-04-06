use std::{env::current_dir, fs, io::Write, path::PathBuf};

mod config;
mod process;
mod server;

fn main() {
    let mut args = std::env::args();
    args.next();
    match args.next().expect("Error: no command given").as_ref() {
        "init" => {
            let dir_error = |name| format!("Error: failed to create directory '{}'", name);
            config::gen_default_file(PathBuf::from("ssg.conf"));
            fs::create_dir("src").expect(dir_error("src").as_str());
            fs::create_dir("static").expect(dir_error("static").as_str());
            fs::create_dir("include").expect(dir_error("include").as_str());
            fs::create_dir("_site").expect(dir_error("_site").as_str());
            let mut head_file = fs::File::create_new("include/head.html")
                .expect("Error: failed to create file 'include/head.html'");
            head_file
                .write(b"<head><meta charset=\"UTF-8\"><title>{title}</title>")
                .unwrap();
            head_file
                .write(b"<link href=\"style.css\" rel=\"stylesheet\"></head>")
                .unwrap();
            head_file.flush().unwrap();
        }
        "build" => {
            let base = find_base_dir();
            process::run(&config::Config::new(base.join("ssg.conf")));
        }
        "server" => {
            let base = find_base_dir();
            let config = config::Config::new(base.join("ssg.conf"));
            process::run(&config);
            println!(
                "Running http server at http://127.0.0.1:{}",
                config.server_port
            );
            server::run_server(&config.server_port, config.abs_site());
        }
        unknown => {
            panic!("Unrecognized command {unknown}");
        }
    }
}

fn find_base_dir() -> PathBuf {
    let mut curdir = current_dir().expect("Error: current directory is invalid");
    loop {
        if curdir.join("ssg.conf").exists() {
            return curdir;
        }

        match curdir.parent() {
            Some(parent) => curdir = parent.to_path_buf(),
            None => panic!(
                "Error: could not find `ssg.conf` in `{:?}` or any parent directory",
                curdir
            ),
        }
    }
}
