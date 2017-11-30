#[macro_use]
extern crate clap;
include!("src/cli.rs");
use clap::Shell;
use std::env;

fn main() {
    let mut app = build_cli();
    app.gen_completions("powerline-rs", Shell::Bash, env::var("CARGO_MANIFEST_DIR").unwrap());
    app.gen_completions("powerline-rs", Shell::Fish, env::var("CARGO_MANIFEST_DIR").unwrap());
}
