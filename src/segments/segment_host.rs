use std::{
    borrow::Cow,
    os::raw::{c_char, c_int},
    str
};
use crate::{Powerline, Segment, Shell};

extern "C" {
    fn gethostname(buf: *mut c_char, len: usize) -> c_int;
}

pub fn segment_host(p: &mut Powerline) {
    let (bg, fg) = (p.theme.hostname_bg, p.theme.hostname_fg);

    if p.shell == Shell::Bare {
        // We don't want to dont_escape() here
        let mut name = [0u8; 256];
        let mut string = Cow::from("error");
        if unsafe { gethostname(&mut name[0] as *mut _ as *mut c_char, name.len()) } == 0 {
            let len = name.iter().position(|i| *i == 0).unwrap_or(name.len());

            if let Ok(name) = str::from_utf8(&name[..len]) {
                string = Cow::from(String::from(name));
            }
        }
        p.segments.push(Segment::new(bg, fg, string));
        return;
    }

    p.segments.push(Segment::new(bg, fg, match p.shell {
        Shell::Bare => unreachable!(),
        Shell::Bash => "\\h",
        Shell::Zsh  => "%m"
    }).dont_escape());
}
