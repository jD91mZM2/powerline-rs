use std::borrow::Cow;
use crate::{Powerline, Segment, Shell};

pub fn segment_linebreak(p: &mut Powerline) {
    let (bg, fg) = (0, 0);
    p.segments.push(match p.shell {
        Shell::Bare => Segment::new(bg, fg, "\n").dont_escape().with_no_space_after(),
        Shell::Bash => Segment::new(bg, fg, "\n").dont_escape().with_no_space_after(),
        Shell::Zsh => Segment::new(bg, fg, "\n").dont_escape().with_no_space_after(),
    });
}
