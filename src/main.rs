#[macro_use]
extern crate clap;

mod cli;
mod format;
mod module;
mod segments;
mod theme;

use crate::module::Module;
use crate::segments::Segment;
use crate::theme::Theme;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Shell {
    Bare,
    Bash,
    Zsh
}

pub struct Powerline {
    segments: Vec<Segment>,
    theme: Theme,
    shell: Shell,

    #[cfg(feature = "git2")]
    git: Option<git2::Repository>,
    #[cfg(feature = "git2")]
    git_statuses: Option<Vec<git2::Status>>
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

    let modules: Vec<_> = matches
        .values_of("modules")
        .unwrap()
        .map(|module| module.parse().unwrap())
        .collect();

    let time_format = matches.value_of("time_format").unwrap();

    #[cfg(feature = "flame")]
    flame::end("parse modules");

    #[cfg(feature = "flame")]
    flame::end("parse arguments");

    #[cfg(feature = "flame")]
    flame::start("main");

    let mut p = Powerline {
        segments: Vec::with_capacity(16), // just a guess
        theme,
        shell: match matches.value_of("shell").unwrap() {
            "bare" => Shell::Bare,
            "bash" => Shell::Bash,
            "zsh"  => Shell::Zsh,
            _ => unreachable!()
        },

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
            Module::Host => segments::segment_host(&mut p),
            Module::Jobs => segments::segment_jobs(&mut p),
            Module::NixShell => segments::segment_nix(&mut p),
            Module::Perms => segments::segment_perms(&mut p),
            Module::Ps => segments::segment_ps(&mut p),
            Module::Root => segments::segment_root(&mut p, error),
            Module::Ssh => segments::segment_ssh(&mut p),
            Module::Time => segments::segment_time(&mut p, &time_format),
            Module::User => segments::segment_user(&mut p),
            Module::LineBreak => segments::segment_linebreak(&mut p),
            Module::VirtualEnv => segments::segment_virtualenv(&mut p),
        }
    }

    #[cfg(feature = "flame")]
    flame::end("main");
    #[cfg(feature = "flame")]
    flame::start("print");

    let n = p.segments.len();
    for i in 1..n+1 {
        p.segments[n-i].escape(p.shell);
        p.segments[n-i].print(p.segments.get(n-i+1), p.shell, &p.theme);
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
