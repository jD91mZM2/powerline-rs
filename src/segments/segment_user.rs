use std::{env, os::raw::c_int};
use {Powerline, Shell, Segment};

#[cfg(unix)]
extern "C" {
    fn getuid() -> c_int;
}

pub fn segment_user(p: &mut Powerline) {
    let (mut bg, fg) = (p.theme.username_bg, p.theme.username_fg);

    #[cfg(unix)]
    {
        if unsafe { getuid() } == 0 {
            bg = p.theme.username_root_bg;
        }
    }

    if p.shell == Shell::Bare {
        // Yeah the optimal approach wouldn't be to use environment variables
        // but then again it would be a lot more code (even if from a library),
        // therefore *probably* slower.
        // But then again I don't know.

        // We don't want to dont_escape() here
        p.segments.push(Segment::new(bg, fg,
            env::var("USER").unwrap_or_else(|_| String::from("error"))));
        return;
    }

    p.segments.push(Segment::new(bg, fg, match p.shell {
        Shell::Bare => unreachable!(),
        Shell::Bash => "\\u",
        Shell::Zsh  => "%n"
    }).dont_escape());
}
