#[macro_use]
extern crate clap;

mod module { include!("src/module.rs"); }
mod cli { include!("src/cli.rs"); }

use clap::Shell;
use std::{fs, env};

fn main() {
    let dir = env::var("COMPLETION_OUT").or_else(|_| env::var("OUT_DIR")).expect("cargo didn't set $OUT_DIR");
    fs::create_dir_all(&dir).expect("failed to create directories");

    let mut app = cli::build_cli();
    app.gen_completions("powerline-rs", Shell::Bash, &dir);
    app.gen_completions("powerline-rs", Shell::Fish, &dir);
}
