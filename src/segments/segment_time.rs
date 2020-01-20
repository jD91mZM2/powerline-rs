#[cfg(feature = "chrono")] use chrono::Local;
#[cfg(feature = "chrono")] use std::fmt::Write;
use crate::{Powerline, Segment, Shell};

pub fn segment_time(p: &mut Powerline, strftime: &str) {
    let (bg, fg) = (p.theme.time_bg, p.theme.time_fg);
    if p.shell == Shell::Bare || strftime != crate::cli::TIME_FORMAT_DEFAULT {
        #[cfg(feature = "chrono")]
        {
            let now = Local::now();
            let mut formatted = String::with_capacity(strftime.len());
            write!(formatted, "{}", now.format(strftime)).unwrap();
            // We don't want to dont_escape() here
            p.segments.push(Segment::new(bg, fg, formatted));
        }
        return;
    }
    p.segments.push(Segment::new(bg, fg, match p.shell {
        Shell::Bare => unreachable!(),
        Shell::Bash => "\\@",
        Shell::Zsh  => "%@"
    }).dont_escape())
}
