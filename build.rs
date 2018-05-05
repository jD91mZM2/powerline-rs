#[macro_use]
extern crate clap;
mod module { include!("src/module.rs"); }
mod cli { include!("src/cli.rs"); }
use clap::Shell;
use std::env;

fn main() {
    let mut app = cli::build_cli();
    app.gen_completions("powerline-rs", Shell::Bash, env::var("CARGO_MANIFEST_DIR").unwrap());
    app.gen_completions("powerline-rs", Shell::Fish, env::var("CARGO_MANIFEST_DIR").unwrap());
}
