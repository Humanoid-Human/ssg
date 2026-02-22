use clap::Parser;

mod walker;
mod process;

#[derive(Parser, Debug)]
struct BuildArgs {
    build_drafts: bool
}

fn main() {
    process::file(std::fs::read_to_string("test.md").unwrap(), std::fs::File::create("test.html").unwrap());
}

fn unknown_command(cmd: &str) {
    println!("Unknown command: {}", cmd);
}
