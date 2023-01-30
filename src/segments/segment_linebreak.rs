use std::borrow::Cow;
use crate::{Powerline, Segment, Shell};

pub fn segment_linebreak(p: &mut Powerline) {
    // Use default bg and fg colors to avoid any coloring (Reset will be applied, see `format.rs`)
    let (bg, fg) = (0, 7);
    p.segments.push(match p.shell {
        Shell::Bare => Segment::new(bg, fg, "\n").dont_escape().with_no_space_after(),
        Shell::Bash => Segment::new(bg, fg, "\n").dont_escape().with_no_space_after(),
        Shell::Zsh => Segment::new(bg, fg, "\n").dont_escape().with_no_space_after(),
    });
}
