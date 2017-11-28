#[macro_use]
extern crate clap;
extern crate termion;

mod format;
mod segment;

use clap::{App, Arg};
use format::*;
use segment::Segment;

fn main() {
    let matches = App::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!())
        .version(crate_version!())
        .arg(
            Arg::with_name("cwd-max-depth")
                .help("Maximum number of directories to show in path")
                .takes_value(true)
                .value_name("int")
                .default_value("5")
        )
        .arg(
            Arg::with_name("cwd-max-dir-size")
                .help("Maximum number of letters displayed for each directory in the path. Setting this to 0 means unlimited.")
                .takes_value(true)
                .value_name("int")
                .default_value("0")
        )
        .arg(
            Arg::with_name("error")
                .help("Exit code of previously executed command")
                .index(3)
                .default_value("0")
        )
        .arg(
            Arg::with_name("max-width")
                .help("Maximum width of the shell that the prompt may use, in percent.\n\
                      Setting this to 0 disables the shrinking subsystem.")
                .takes_value(true)
                .value_name("int")
                .default_value("50")
        )
        .arg(
            Arg::with_name("mode")
                .help("The characters used to make separators between segments.")
                .takes_value(true)
                .value_name("string")
                .possible_values(&["patched", "compatible", "flat"])
                .default_value("patched")
        )
        .arg(
            Arg::with_name("modules")
                .long_help("The list of modules to load, separated by ','\n\
(Valid modules: aws, cwd, docker, dotenv, exit, git, gitlite, hg, host, jobs, perlbrew, perms, root, ssh, time, user, venv)")
                .takes_value(true)
                .value_name("string")
                .default_value("ssh,cwd,perms,git,hg,jobs,exit,root")
        )
        .arg(
            Arg::with_name("shell")
                .help("Set this to your shell type")
                .takes_value(true)
                .value_name("string")
                .possible_values(&["bare", "bash", "zsh"])
                .default_value("bare")
        )
        .get_matches();

    let mut segments = Vec::new();

    segments.push(Segment::new(HOME_BG, HOME_FG, "~"));
    for test in &["test", "lol", "woot"] {
        segments.push(Segment::new(PATH_BG, PATH_FG, *test));
    }

    for i in 0..segments.len() {
        segments[i].print(segments.get(i+1));
    }
}
