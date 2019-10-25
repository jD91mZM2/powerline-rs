use crate::{Powerline, Segment};
use std::{
    fmt::Write,
    fs,
    os::raw::c_int,
    path::Path
};

const PROC_STAT_PID: usize = 6; // 0-based, 7th word

extern "C" {
    fn getpid() -> c_int; // std::process::id() is unstable
}

pub fn segment_ps(p: &mut Powerline) {
    let pid = unsafe { getpid() };
    let tty = {
        let mut path = String::with_capacity(6 + 4 + 5); // 4 = reserved pid length
        path.push_str("/proc/");
        write!(path, "{}", pid).unwrap();
        path.push_str("/stat");

        match get_process_tty(&Path::new(&path)) {
            Some(tty) => tty,
            None => return
        }
    };

    let mut count = -1isize;

    if let Ok(list) = fs::read_dir("/proc/") {
        for entry in list {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => break
            };
            let mut path = entry.path();

            if !path.file_name()
                    .and_then(|name| name
                        .to_str()
                        .map(|s| {
                            s.chars().all(|c| c >= '0' && c <= '9')
                                && s.parse() != Ok(pid)
                        }))
                    .unwrap_or(false) {
                continue;
            }

            path.push("stat");

            if get_process_tty(&path) == Some(tty) {
                count += 1;
            }
        }
    }

    if count > 0 {
        p.segments.push(Segment::new(p.theme.ps_bg, p.theme.ps_fg, count.to_string()));
    }
}
pub fn get_process_tty(file: &Path) -> Option<usize> {
    fs::read_to_string(&file).ok()?
        .split_whitespace().nth(PROC_STAT_PID)
        .and_then(|n| n.parse().ok())
}
