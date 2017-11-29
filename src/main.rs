#[macro_use]
extern crate clap;
extern crate git2;
extern crate termion;

mod format;
mod module;
mod segment;

use clap::{App, Arg};
use format::*;
use git2::{BranchType, ObjectType, Repository, StatusOptions, StatusShow};
use module::Module;
use segment::Segment;
use std::collections::VecDeque;
use std::env;
use std::path::PathBuf;

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
            Module::Cwd => {
                let mut path = env::current_dir().unwrap_or_else(|_| PathBuf::from("error"));
                if let Some(home) = env::home_dir() {
                    let mut new_path = None;
                    if let Ok(new) = path.strip_prefix(&home) {
                        segments.push_back(Segment::new(HOME_BG, HOME_FG, "~"));
                        // TODO: When non-lexical lifetimes are a thing, use drop(path) here.
                        new_path = Some(new.to_path_buf());
                    }
                    // Bypass borrow checker kek
                    if let Some(new) = new_path {
                        path = new;
                    }
                }

                let length = path.iter().count();
                let mut depth  = length;
                let mut shortened = false;

                let cwd_max_depth = cwd_max_depth as usize;

                for (i, path) in path.iter().enumerate() {
                    let fg = if i == length-1 { CWD_FG } else { PATH_FG };

                    if cwd_max_depth > 0 && (i != 0 || cwd_max_depth == 1) && i != length-1 && depth > cwd_max_depth {
                        if !shortened { // First time
                            segments.push_back(Segment::new(PATH_BG, fg, String::from("…")));
                            shortened = true;
                        } else {
                            depth -= 1;
                        }
                    } else {
                        let mut path = path.to_string_lossy().into_owned();

                        let cwd_max_dir_size = cwd_max_dir_size as usize;
                        if cwd_max_dir_size > 0 && path.len() > cwd_max_dir_size {
                            path = String::from(&path[..cwd_max_dir_size]);
                            path.push('…');
                        }
                        segments.push_back(Segment::new(PATH_BG, fg, path));
                    }
                }
            },
            Module::Git => {
                if git.is_none() {
                    continue;
                }
                let git = git.as_ref().unwrap();

                let branches = git.branches(Some(BranchType::Local));
                if branches.is_err() {
                    continue;
                }

                let mut current = None;

                for branch in branches.unwrap() {
                    if let Ok((branch, _)) = branch {
                        if branch.is_head() {
                            if let Ok(name) = branch.name() {
                                if let Some(name) = name {
                                    current = Some(name.to_string());
                                    break;
                                }
                            }
                        }
                    }
                }

                if current.is_none() {
                    // Could be a detached head
                    if let Ok(head) = git.head() {
                        if let Some(target) = head.target() {
                            current = git.find_object(target, Some(ObjectType::Any))
                                        .ok()
                                        .and_then(|obj| obj.short_id().ok())
                                        .and_then(|buf| buf.as_str()
                                                            .map(|s| s.to_string()))
                        }
                    } else {
                        segments.push_back(Segment::new(REPO_DIRTY_BG, REPO_DIRTY_FG, "Big Bang"));
                        continue;
                    }
                }

                let statuses = git.statuses(Some(
                    StatusOptions::new()
                        .show(StatusShow::IndexAndWorkdir)
                        .include_untracked(true)
                ));
                if statuses.is_err() {
                    continue;
                }

                let (mut bg, mut fg) = (REPO_DIRTY_BG, REPO_DIRTY_FG);
                if statuses.unwrap().len() == 0 {
                    bg = REPO_CLEAN_BG;
                    fg = REPO_CLEAN_FG;
                }
                segments.push_back(Segment::new(bg, fg, current.unwrap()));
            },
            Module::GitStage => {
                if git.is_none() {
                    continue;
                }
                let git = git.as_ref().unwrap();

                let statuses = git.statuses(Some(
                    StatusOptions::new()
                        .show(StatusShow::IndexAndWorkdir)
                        .include_untracked(true)
                        .renames_from_rewrites(true)
                        .renames_head_to_index(true)
                ));
                if statuses.is_err() {
                    continue;
                }

                let mut staged = 0;
                let mut notstaged = 0;
                let mut untracked = 0;
                let mut conflicted = 0;

                for status in statuses.unwrap().iter() {
                    let status = status.status();
                    if status.contains(git2::STATUS_INDEX_NEW)
                        || status.contains(git2::STATUS_INDEX_MODIFIED)
                        || status.contains(git2::STATUS_INDEX_TYPECHANGE)
                        || status.contains(git2::STATUS_INDEX_RENAMED)
                        || status.contains(git2::STATUS_INDEX_DELETED) {
                        staged += 1;
                    }
                    if status.contains(git2::STATUS_WT_MODIFIED)
                        || status.contains(git2::STATUS_WT_TYPECHANGE)
                        || status.contains(git2::STATUS_WT_DELETED) {
                        notstaged += 1;
                    }
                    if status.contains(git2::STATUS_WT_NEW) {
                        untracked += 1;
                    }
                    if status.contains(git2::STATUS_CONFLICTED) {
                        conflicted += 1;
                    }
                }

                if staged > 0 {
                    let mut string = if staged == 1 { String::with_capacity(1) } else { staged.to_string() };
                    string.push('✔');
                    segments.push_back(Segment::new(GIT_STAGED_BG, GIT_STAGED_FG, string));
                }
                if notstaged > 0 {
                    let mut string = if notstaged == 1 { String::with_capacity(1) } else { notstaged.to_string() };
                    string.push('✎');
                    segments.push_back(Segment::new(GIT_NOTSTAGED_BG, GIT_NOTSTAGED_FG, string));
                }
                if untracked > 0 {
                    let mut string = if untracked == 1 { String::with_capacity(1) } else { untracked.to_string() };
                    string.push('+');
                    segments.push_back(Segment::new(GIT_UNTRACKED_BG, GIT_UNTRACKED_FG, string));
                }
                if conflicted > 0 {
                    let mut string = if conflicted == 1 { String::with_capacity(1) } else { conflicted.to_string() };
                    string.push('*');
                    segments.push_back(Segment::new(GIT_CONFLICTED_BG, GIT_CONFLICTED_FG, string));
                }
            },
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
