#[cfg(feature = "chrono")] use chrono::Local;
#[cfg(feature = "chrono")] use chrono::prelude::*;
#[cfg(feature = "chrono")] use std::fmt::Write;
use crate::{Powerline, Segment, Shell};

pub fn segment_time(p: &mut Powerline) {
    let (bg, fg) = (p.theme.time_bg, p.theme.time_fg);
    if p.shell == Shell::Bare {
        #[cfg(feature = "chrono")]
        {
            let time = Local::now();

            let (ampm, hour) = time.hour12();
            let ampm = if ampm { "AM" } else { "PM" };

            let mut formatted = String::with_capacity(2 + 1 + 2 + 1 + 2);
            write!(formatted, "{:02}:{:02} {}", hour, time.minute(), ampm).unwrap();

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
