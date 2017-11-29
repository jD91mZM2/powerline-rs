#[macro_use]
extern crate clap;
extern crate git2;
extern crate termion;

mod format;
mod module;
mod segment;
mod segments;

use clap::{App, Arg};
use format::*;
use git2::Repository;
use module::Module;
use segment::Segment;
use std::collections::VecDeque;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Shell {
    Bare,
    Bash,
    Zsh
}

fn main() {
    let matches = App::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!())
        .version(crate_version!())
        .arg(
            Arg::with_name("cwd-max-depth")
                .long("cwd-max-depth")
                .help("Maximum number of directories to show in path")
                .takes_value(true)
                .value_name("int")
                .default_value("5")
        )
        .arg(
            Arg::with_name("cwd-max-dir-size")
                .long("cwd-max-dir-size")
                .help("Maximum number of letters displayed for each directory in the path. Setting this to 0 means unlimited.")
                .takes_value(true)
                .value_name("int")
                .default_value("0")
        )
        .arg(
            Arg::with_name("error")
                .help("Exit code of previously executed command")
                .default_value("0")
        )
        .arg(
            Arg::with_name("max-width")
                .long("max-width")
                .help("Maximum width of the shell that the prompt may use, in percent.\n\
                      Setting this to 0 disables the shrinking subsystem.")
                .takes_value(true)
                .value_name("int")
                .default_value("50")
        )
        .arg(
            Arg::with_name("modules")
                .long("modules")
                .long_help("The list of modules to load, separated by ','\n\
(Valid modules: aws, cwd, docker, dotenv, exit, git, gitstage, gittrack, hg, host, jobs, perlbrew, perms, root, ssh, time, user, venv)")
                .takes_value(true)
                .value_name("string")
                .default_value("ssh,cwd,perms,git,gitstage,hg,jobs,exit,root")
        )
        .arg(
            Arg::with_name("shell")
                .long("shell")
                .help("Set this to your shell type")
                .takes_value(true)
                .value_name("string")
                .possible_values(&["bare", "bash", "zsh"])
                .default_value("bash")
        )
        .get_matches();

    macro_rules! parse {
        ($name:expr) => {
            match matches.value_of($name).unwrap().parse::<u8>() {
                Ok(value) => value,
                Err(_)  => {
                    eprintln!(concat!("Value of --", $name, " isn't a valid number."));
                    return;
                }
            }
        }
    }

    let cwd_max_depth    = parse!("cwd-max-depth");
    let cwd_max_dir_size = parse!("cwd-max-dir-size");
    let error            = parse!("error");
    let max_width        = parse!("max-width");

    let max_width = if let Ok((width, _)) = termion::terminal_size() {
        ((max_width as f32 / 100.0) * width as f32) as u16
    } else {
        0
    };

    let modules_iter = matches.value_of("modules").unwrap()
                            .split(",")
                            .map(|module| module.parse::<Module>());
    let mut modules = Vec::with_capacity(16); // just a guess
    for module in modules_iter {
        if module.is_err() {
            eprintln!("Module string invalid!");
            return;
        }
        modules.push(module.unwrap());
    }
    modules.shrink_to_fit();

    let shell = match matches.value_of("shell").unwrap() {
        "bare" => Shell::Bare,
        "bash" => Shell::Bash,
        "zsh"  => Shell::Zsh,
        _ => unreachable!()
    };

    let git = if modules.iter().any(|module| *module == Module::Git || *module == Module::GitStage) {
        Repository::discover(".").ok()
    } else { None };

    let mut segments = VecDeque::new();

    for module in modules {
        match module {
            Module::Cwd => segments::segment_cwd(&mut segments, cwd_max_depth, cwd_max_dir_size),
            Module::Git => segments::segment_git(&mut segments, &git),
            Module::GitStage => segments::segment_gitstage(&mut segments, &git),
            Module::Root => {
                let (mut bg, mut fg) = (CMD_PASSED_BG, CMD_PASSED_FG);
                if error != 0 {
                    bg = CMD_FAILED_BG;
                    fg = CMD_FAILED_FG;
                }
                segments.push_back(Segment::new(bg, fg, root(shell)));
            },
            _ => () // unimplemented!()
        }
    }
    if max_width != 0 {
        loop {
            let mut total = 0;
            for segment in &segments {
                total += segment.len();
                total += 1;
            }

            if total < max_width as usize {
                break;
            }
            segments.pop_front();
        }
    }
    for i in 0..segments.len() {
        segments[i].print(segments.get(i+1), shell);
    }
    print!(" ");
}
