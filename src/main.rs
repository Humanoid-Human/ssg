use clap::Parser;

mod walker;
mod process;

#[derive(Parser, Debug)]
struct BuildArgs {
    build_drafts: bool
}

fn main() {
    
}

fn unknown_command(cmd: &str) {
    println!("Unknown command: {}", cmd);
}
