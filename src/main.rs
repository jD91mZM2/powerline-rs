#[cfg(feature = "chrono")] extern crate chrono;
#[macro_use] extern crate clap;
#[cfg(feature = "git2")] extern crate git2;
#[cfg(feature = "flame")] extern crate flame;

mod cli;
mod format;
mod module;
mod segments;
mod theme;

#[cfg(feature = "chrono")] use chrono::Local;
#[cfg(feature = "chrono")] use chrono::prelude::*;
#[cfg(feature = "chrono")] use std::fmt::Write;
#[cfg(unix)] use std::ffi::CString;
#[cfg(unix)] use std::os::raw::*;
use format::*;
use module::Module;
use segments::Segment;
use std::env;
use theme::Theme;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Shell {
    Bare,
    Ion,
    Bash,
    Zsh
}

pub struct Powerline {
    segments: Vec<Segment>,
    theme: Theme,

    #[cfg(feature = "git2")]
    git: Option<git2::Repository>,
    #[cfg(feature = "git2")]
    git_statuses: Option<Vec<git2::Status>>
}

#[cfg(unix)]
extern "C" {
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn getuid() -> c_int;
    fn getpid() -> c_int; // std::process::id() is unstable
}

fn main() {
    #[cfg(feature = "flame")]
    flame::start("clap-rs");

    let matches = cli::build_cli().get_matches();

    #[cfg(feature = "flame")]
    flame::end("clap-rs");

    #[cfg(feature = "flame")]
    flame::start("parse arguments");

    let cwd_max_depth    = value_t_or_exit!(matches, "cwd-max-depth", u8);
    let cwd_max_dir_size = value_t_or_exit!(matches, "cwd-max-dir-size", u8);
    let error            = value_t_or_exit!(matches, "error", u8);

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
        "ion"  => Shell::Ion,
        "zsh"  => Shell::Zsh,
        _ => unreachable!()
    };

    #[cfg(feature = "flame")]
    flame::end("parse arguments");

    #[cfg(feature = "flame")]
    flame::start("main");

    let mut p = Powerline {
        segments: Vec::with_capacity(16), // just a guess
        theme: theme,

        #[cfg(feature = "git2")]
        git: None,
        #[cfg(feature = "git2")]
        git_statuses: None
    };

    for module in modules {
        match module {
            Module::Cwd => segments::segment_cwd(&mut p, cwd_max_depth, cwd_max_dir_size),
            Module::Git => { #[cfg(feature = "git2")] segments::segment_git(&mut p) },
            Module::GitStage => { #[cfg(feature = "git2")] segments::segment_gitstage(&mut p) },
            Module::Ps => segments::segment_ps(&mut p),
            Module::Host => {
                let (bg, fg) = (p.theme.hostname_bg, p.theme.hostname_fg);

                if shell == Shell::Bare {
                    p.segments.push(
                        Segment::new(bg, fg, env::var("HOSTNAME")
                            .unwrap_or_else(|_| String::from("error")))
                    );
                    continue;
                }
                p.segments.push(Segment::new(bg, fg, match shell {
                    Shell::Bare => unreachable!(),
                    Shell::Bash => "\\h",
                    Shell::Ion  => "\\h",
                    Shell::Zsh  => "%m"
                }).dont_escape());
            },
            Module::Jobs => {
                p.segments.push(match shell {
                    Shell::Bare => continue,
                    Shell::Bash =>
                        Segment::new(p.theme.jobs_bg, p.theme.jobs_fg, "\\j")
                            .with_before(r#"$(test -n "$(jobs -p)" && echo -n ""#)
                            .with_after(r#"")"#),
                    Shell::Ion  => 
                        Segment::new(p.theme.jobs_bg, p.theme.jobs_fg, "\\j")
                            .with_before(r#"$(test -n "$(jobs -p)" && echo -n ""#)
                            .with_after(r#"")"#), 
                    Shell::Zsh =>
                        Segment::new(p.theme.jobs_bg, p.theme.jobs_fg, "%j")
                            .with_before("%(1j.")
                            .with_after(".)"),
                }.as_conditional().dont_escape());
            },
            Module::Perms => {
                #[cfg(unix)]
                {
                    let path = CString::new(".").unwrap();
                    if unsafe { access(path.as_ptr(), 0x2) } != 0 {
                        p.segments.push(Segment::new(p.theme.ro_bg, p.theme.ro_fg, ""));
                    }
                }
            },
            Module::Ssh => {
                if env::var("SSH_CLIENT").is_ok() {
                    p.segments.push(Segment::new(p.theme.ssh_bg, p.theme.ssh_fg, ""));
                }
            },
            Module::Time => {
                let (bg, fg) = (p.theme.time_bg, p.theme.time_fg);
                if shell == Shell::Bare {
                    #[cfg(feature = "chrono")]
                    {
                        let time = Local::now();

                        let (ampm, hour) = time.hour12();
                        let ampm = if ampm { "AM" } else { "PM" };

                        let mut formatted = String::with_capacity(2 + 1 + 2 + 1 + 2);
                        write!(formatted, "{:02}:{:02} {}", hour, time.minute(), ampm).unwrap();

                        p.segments.push(Segment::new(bg, fg, formatted));
                    }
                    continue;
                }
                p.segments.push(Segment::new(bg, fg, match shell {
                    Shell::Bare => unreachable!(),
                    Shell::Bash => "\\@",
                    Shell::Ion  => "\\@",
                    Shell::Zsh  => "%@"
                }).dont_escape())
            },
            Module::User => {
                let (mut bg, fg) = (p.theme.username_bg, p.theme.username_fg);

                #[cfg(unix)]
                {
                    if unsafe { getuid() } == 0 {
                        bg = p.theme.username_root_bg;
                    }
                }

                if shell == Shell::Bare {
                    // Yeah the optimal approach wouldn't be to use environment variables
                    // but then again it would be a lot more code (even if from a library),
                    // therefore *probably* slower.
                    // But then again I don't know.
                    p.segments.push(Segment::new(bg, fg,
                        env::var("USER").unwrap_or_else(|_| String::from("error"))));
                    continue;
                }
                p.segments.push(Segment::new(bg, fg, match shell {
                    Shell::Bare => unreachable!(),
                    Shell::Bash => "\\u",
                    Shell::Ion  => "\\u",
                    Shell::Zsh  => "%n"
                }).dont_escape());
            },
            Module::Root => {
                let (mut bg, mut fg) = (p.theme.cmd_passed_bg, p.theme.cmd_passed_fg);
                if error != 0 {
                    bg = p.theme.cmd_failed_bg;
                    fg = p.theme.cmd_failed_fg;
                }
                p.segments.push(Segment::new(bg, fg, root(shell)).dont_escape());
            },
        }
    }

    #[cfg(feature = "flame")]
    flame::end("main");
    #[cfg(feature = "flame")]
    flame::start("print");

    for i in 0..p.segments.len() {
        p.segments[i].escape(shell);
        p.segments[i].print(p.segments.get(i+1), shell, &p.theme);
    }

    if matches.is_present("newline") {
        println!();
    } else {
        print!(" ");
    }

    #[cfg(feature = "flame")]
    flame::end("print");

    #[cfg(feature = "flame")]
    {
        use std::fs::File;
        flame::dump_html(&mut File::create("profile.html").unwrap()).unwrap();
    }
}
