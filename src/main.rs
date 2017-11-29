#[macro_use]
extern crate clap;
#[cfg(feature = "git2")]
extern crate git2;
#[cfg(feature = "flame")]
extern crate flame;

mod format;
mod module;
mod segment;
mod segments;
mod theme;

use clap::{App, Arg};
use format::*;
#[cfg(feature = "git2")]
use git2::Repository;
use module::Module;
use segment::Segment;
use std::env;
use std::ffi::CString;
use std::os::raw::*;
use std::time::UNIX_EPOCH;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Shell {
    Bare,
    Bash,
    Zsh
}

#[cfg(unix)]
extern "C" {
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn getuid() -> c_int;
}

fn main() {
    #[cfg(feature = "flame")]
    flame::start("clap-rs");

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
                .default_value("15")
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
                .help("The list of modules to load, separated by ','")
                .takes_value(true)
                .value_name("string")
                .possible_values(&["cwd", "git", "gitstage", "host", "jobs", "perms", "root", "ssh", "time", "user"])
                .value_delimiter(",")
                .default_value("ssh,cwd,perms,git,gitstage,root")
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
        .arg(
            Arg::with_name("theme")
                .long("theme")
                .help("Set this to the theme you want to use")
                .takes_value(true)
                .value_name("file")
        )
        .get_matches();

    #[cfg(feature = "flame")]
    flame::end("clap-rs");

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

    #[cfg(feature = "flame")]
    flame::start("parse arguments");

    let cwd_max_depth    = parse!("cwd-max-depth");
    let cwd_max_dir_size = parse!("cwd-max-dir-size");
    let error            = parse!("error");

    #[cfg(feature = "flame")]
    flame::start("parse theme");

    let theme = if let Some(file) = matches.value_of("theme") {
        if let Ok(theme) = theme::load(file) {
            theme
        } else {
            eprintln!("Invalid theme.");
            theme::DEFAULT
        }
    } else { theme::DEFAULT };

    #[cfg(feature = "flame")]
    flame::end("parse theme");

    #[cfg(feature = "flame")]
    flame::start("parse modules");

    let modules: Vec<_> = matches.values_of("modules").unwrap()
                            .map(|module| module.parse().unwrap())
                            .collect();

    #[cfg(feature = "flame")]
    flame::end("parse modules");

    let shell = match matches.value_of("shell").unwrap() {
        "bare" => Shell::Bare,
        "bash" => Shell::Bash,
        "zsh"  => Shell::Zsh,
        _ => unreachable!()
    };

    #[cfg(feature = "flame")]
    flame::end("parse arguments");

    #[cfg(feature = "flame")]
    flame::start("git discover");

    #[cfg(feature = "git2")]
    let git = if modules.iter().any(|module| *module == Module::Git || *module == Module::GitStage) {
        Repository::discover(".").ok()
    } else { None };

    #[cfg(feature = "flame")]
    flame::end("git discover");

    #[cfg(feature = "flame")]
    flame::start("main");

    let mut segments = Vec::with_capacity(16); // just a guess

    for module in modules {
        match module {
            Module::Cwd => segments::segment_cwd(&mut segments, &theme, cwd_max_depth, cwd_max_dir_size),
            Module::Git => { #[cfg(feature = "git2")] segments::segment_git(&mut segments, &theme, &git) },
            Module::GitStage => { #[cfg(feature = "git2")] segments::segment_gitstage(&mut segments, &theme, &git) },
            Module::Host => {
                let (bg, fg) = (theme.hostname_bg, theme.hostname_fg);

                if shell == Shell::Bare {
                    segments.push(Segment::new(bg, fg, env::var("HOSTNAME")
                                                                .unwrap_or_else(|_| String::from("error"))));
                    continue;
                }
                segments.push(Segment::new(bg, fg, match shell {
                    Shell::Bare => unreachable!(),
                    Shell::Bash => "\\h",
                    Shell::Zsh  => "%m"
                }).dont_escape());
            },
            Module::Jobs => {
                if shell == Shell::Bare { continue; }
                segments.push(Segment::new(theme.jobs_bg, theme.jobs_fg, match shell {
                    Shell::Bare => unreachable!(),
                    Shell::Bash => "\\j",
                    Shell::Zsh  => "%j"
                }).dont_escape());
            },
            Module::Perms => {
                #[cfg(unix)]
                {
                    let path = CString::new(".").unwrap();
                    if unsafe { access(path.as_ptr(), 0x2) } != 0 {
                        segments.push(Segment::new(theme.ro_bg, theme.ro_fg, ""));
                    }
                }
            },
            Module::Ssh => {
                if env::var("SSH_CLIENT").is_ok() {
                    segments.push(Segment::new(theme.ssh_bg, theme.ssh_fg, ""));
                }
            },
            Module::Time => {
                let (bg, fg) = (theme.time_bg, theme.time_fg);
                if shell == Shell::Bare {
                    if let Ok(duration) = UNIX_EPOCH.elapsed() {
                        let secs = duration.as_secs();
                        let mut hours = (secs / 60 / 60) % 24;
                        let mins = (secs / 60) % 60;

                        println!("{}", hours);

                        let ampm = if hours > 12 {
                            hours -= 12;
                            "PM"
                        } else { "AM" };

                        segments.push(Segment::new(bg, fg, format!("{:02}:{:02} {}", hours, mins, ampm)));
                    }
                    continue;
                }
                segments.push(Segment::new(bg, fg, match shell {
                    Shell::Bare => unreachable!(),
                    Shell::Bash => "\\@",
                    Shell::Zsh  => "%@"
                }).dont_escape())
            },
            Module::User => {
                let (mut bg, fg) = (theme.username_bg, theme.username_fg);

                #[cfg(unix)]
                {
                    if unsafe { getuid() } == 0 {
                        bg = theme.username_root_bg;
                    }
                }

                if shell == Shell::Bare {
                    // Yeah the optimal approach wouldn't be to use environment variables
                    // but then again it would be a lot more code (even if from a library),
                    // therefore *probably* slower.
                    // But then again I don't know.
                    segments.push(Segment::new(bg, fg,
                        env::var("USER").unwrap_or_else(|_| String::from("error"))));
                    continue;
                }
                segments.push(Segment::new(bg, fg, match shell {
                    Shell::Bare => unreachable!(),
                    Shell::Bash => "\\u",
                    Shell::Zsh  => "%n"
                }).dont_escape());
            },
            Module::Root => {
                let (mut bg, mut fg) = (theme.cmd_passed_bg, theme.cmd_passed_fg);
                if error != 0 {
                    bg = theme.cmd_failed_bg;
                    fg = theme.cmd_failed_fg;
                }
                segments.push(Segment::new(bg, fg, root(shell)).dont_escape());
            }
        }
    }

    #[cfg(feature = "flame")]
    flame::end("main");
    #[cfg(feature = "flame")]
    flame::start("print");

    for i in 0..segments.len() {
        segments[i].escape(shell);
        segments[i].print(segments.get(i+1), shell, &theme);
    }

    print!(" ");

    #[cfg(feature = "flame")]
    flame::end("print");

    #[cfg(feature = "flame")]
    {
        use std::fs::File;
        flame::dump_html(&mut File::create("profile.html").unwrap()).unwrap();
    }
}
