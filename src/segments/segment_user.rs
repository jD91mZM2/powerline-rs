use std::{
    borrow::Cow,
    os::raw::{c_char, c_int},
    str
};
use {Powerline, Shell, Segment};

extern "C" {
    fn getuid() -> c_int;
    fn getlogin_r(buf: *mut c_char, len: usize) -> c_int;
}

pub fn segment_user(p: &mut Powerline) {
    let (mut bg, fg) = (p.theme.username_bg, p.theme.username_fg);

    if unsafe { getuid() } == 0 {
        bg = p.theme.username_root_bg;
    }

    if p.shell == Shell::Bare {
        // Yeah the optimal approach wouldn't be to use environment variables
        // but then again it would be a lot more code (even if from a library),
        // therefore *probably* slower.
        // But then again I don't know.

        // We don't want to dont_escape() here
        let mut name = [0u8; 256];
        let mut string = Cow::from("error");
        if unsafe { getlogin_r(&mut name[0] as *mut _ as *mut c_char, name.len()) } == 0 {
            let mut len = 0;
            while name[len] != 0 {
                len += 1;
            }
            if let Ok(name) = str::from_utf8(&name[..len]) {
                string = Cow::from(String::from(name));
            }
        }
        p.segments.push(Segment::new(bg, fg, string));
        return;
    }

    p.segments.push(Segment::new(bg, fg, match p.shell {
        Shell::Bare => unreachable!(),
        Shell::Bash => "\\u",
        Shell::Zsh  => "%n"
    }).dont_escape());
}
