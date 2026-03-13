use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
    fs,
};

const OK: &str = "HTTP/1.1 200 OK";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND";

pub fn run_server(port: &str, base_dir: PathBuf) {
    let tcp = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();

    for stream in tcp.incoming() {
        handle_connection(stream.unwrap(), &base_dir);
    }
}

fn handle_connection(mut stream: TcpStream, base_dir: &Path) {
    let request = BufReader::new(&stream).lines().next().unwrap().unwrap();
    let mut parts = request.split(" ");
    let first = parts.next();
    if first.is_none() { return; }

    let first = first.unwrap();
    let path = &parts.next().unwrap()[1..];

    // TODO: Content-Type
    if first == "GET"
        && let Some((len, contents)) = get_file(base_dir.join(path)) {
        let header = format!("{OK}\r\nContent-Length:{len}\r\n\r\n");
        let _ = stream.write_all(header.as_bytes());
        let _ = stream.write_all(&contents);
    } else {
        let _ = stream.write_all(&not_found(base_dir));
    }
}

fn not_found(base_dir: &Path) -> Vec<u8> {
    let mut out = format!("{NOT_FOUND}\r\n").into_bytes();
    if let Some((len, mut contents)) = get_file(base_dir.join("404.html")) {
        let info = format!("Content-Length:{len}\r\nContent-Type:text/html; charset=UTF-8");
        out.append(&mut info.into_bytes());
        out.append(&mut contents);
    }

    out
}

fn get_file(mut p: PathBuf) -> Option<(usize, Vec<u8>)> {
    if !p.exists() {
        if p.extension().is_none() {
            p.set_extension("html");
            return get_file(p);
        } else {
            return None;
        }
    }

    if p.is_dir() {
        let index = p.join("index.html");
        if index.exists() {
            return get_file(index);
        } else {
            p.set_extension("html");
            return get_file(p);
        }
        
    }

    if let Ok(contents) = fs::read(p) {
        Some((contents.len(), contents))
    } else {
        None
    }
}