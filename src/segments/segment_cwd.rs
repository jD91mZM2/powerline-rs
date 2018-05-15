use {Powerline, Segment};
use std::env;
use std::path::PathBuf;

pub fn segment_cwd(p: &mut Powerline, cwd_max_depth: u8, cwd_max_dir_size: u8) {
    let mut path = env::current_dir().unwrap_or_else(|_| PathBuf::from("error"));
    if let Some(home) = env::home_dir() {
        let mut new_path = None;
        if let Ok(new) = path.strip_prefix(&home) {
            p.segments.push(Segment::new(p.theme.home_bg, p.theme.home_fg, "~"));
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
        let fg = if i == length-1 { p.theme.cwd_fg } else { p.theme.path_fg };

        if cwd_max_depth > 0 && (i != 0 || cwd_max_depth == 1) && i != length-1 && depth > cwd_max_depth {
            if !shortened { // First time
                p.segments.push(Segment::new(p.theme.path_bg, fg, String::from("…")));
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
            p.segments.push(Segment::new(p.theme.path_bg, fg, path));
        }
    }
}
