// use std::env;
use clap::Parser;

mod build;
mod walker;
mod process;

#[derive(Parser, Debug)]
struct BuildArgs {
    build_drafts: bool
}

fn main() {
    env::args().next();
    let cmd = env::args().next().expect("No command provided. See 'ssg help'.");
    let args: Vec<String> = env::args().collect();
    match cmd.as_str() {
        "build" => build::build(BuildArgs::parse().build_drafts),
        _ => unknown_command(&cmd)
    }
}

fn unknown_command(cmd: &str) {
    println!("Unknown command: {}", cmd);
}
