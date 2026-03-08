use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    fs::read_to_string
};

const OK: &str = "HTTP/1.1 200 OK";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND";

pub fn run_server(port: &str, base_dir: PathBuf) {
    let tcp = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();

    for stream in tcp.incoming() {
        handle_connection(stream.unwrap(), &base_dir);
    }
}

fn handle_connection(mut stream: TcpStream, base_dir: &PathBuf) {
    let request = BufReader::new(&stream).lines().next().unwrap().unwrap();
    let mut parts = request.split(" ");
    let first = parts.next();

    // TODO: image handling
    let output: String;
    if first.is_none() || first.unwrap() != "GET" {
        output = not_found(base_dir);
    } else {
        let (len, contents) = get_file(&base_dir.join(parts.next().unwrap()));
        output = format!("{OK}\r\nContent-Length:{len}\r\n\r\n{contents}");
    }

    let _ = stream.write_all(output.as_bytes());
}

fn not_found(base_dir: &PathBuf) -> String {
    let (len, contents) = get_file(&base_dir.join("404.html"));

    return format!("{NOT_FOUND}\r\nContent-Length:{len}\r\n\r\n{contents}")
}

fn get_file(p: &PathBuf) -> (usize, String) {
    if !p.exists() || !p.is_file() {
        return (0, "".to_string());
    }

    if let Ok(contents) = read_to_string(p) {
        (contents.len(), contents)
    } else {
        (0, "".to_string())
    }
}