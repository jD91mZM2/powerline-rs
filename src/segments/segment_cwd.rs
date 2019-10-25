use crate::{Powerline, Segment};
use std::{
    borrow::Cow,
    env,
    ffi::OsStr,
    path::PathBuf
};

pub fn segment_cwd(p: &mut Powerline, cwd_max_depth: u8, cwd_max_dir_size: u8) {
    let mut path = env::current_dir().unwrap_or_else(|_| PathBuf::from("error"));
    if let Some(home) = dirs::home_dir() {
        let mut new_path = None;
        if let Ok(new) = path.strip_prefix(&home) {
            p.segments.push(Segment::new(p.theme.home_bg, p.theme.home_fg, "~"));
            // TODO: NLL: path = new.to_path_buf();
            new_path = Some(new.to_path_buf());
        }
        if let Some(new) = new_path {
            path = new;
        }
    }

    let length = path.iter().count();
    let mut dirs = path.iter();

    let cwd_max_depth = cwd_max_depth as usize;

    if cwd_max_depth != 1 {
        if let Some(dir) = dirs.next() {
            // Either there's no cwd_max_depth, or it's bigger than 1
            segment(p, dir, length == 1, cwd_max_dir_size);

            // It would be sane here to subtract 1 from both length and
            // cwd_max_depth, to make it clear that we already tried one and
            // what the below code is doing. HOWEVER, currently that results in
            // the exact same outcome.
        }
    }
    if cwd_max_depth > 0 && length > cwd_max_depth {
        p.segments.push(Segment::new(p.theme.path_bg, p.theme.path_fg, Cow::from("…")));

        for _ in 0..length - cwd_max_depth {
            dirs.next().unwrap();
        }
    }

    let mut next = dirs.next();
    while let Some(cursor) = next {
        next = dirs.next();

        segment(p, cursor, next.is_none(), cwd_max_dir_size);
    }
}
pub fn segment(p: &mut Powerline, name: &OsStr, last: bool, cwd_max_dir_size: u8) {
    let mut name = name.to_string_lossy().into_owned();

    let cwd_max_dir_size = cwd_max_dir_size as usize;
    if cwd_max_dir_size > 0 && name.chars().count() > cwd_max_dir_size {
        let mut start = 0;
        for c in name.chars().take(cwd_max_dir_size) {
            start += c.len_utf8();
        }
        name.drain(start..);
        name.push('…');
    }

    let fg = if last { p.theme.cwd_fg } else { p.theme.path_fg };
    p.segments.push(Segment::new(p.theme.path_bg, fg, name));
}
