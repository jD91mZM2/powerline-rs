use {format, Powerline, Segment};

pub fn segment_root(p: &mut Powerline, error: u8) {
    let (mut bg, mut fg) = (p.theme.cmd_passed_bg, p.theme.cmd_passed_fg);
    if error != 0 {
        bg = p.theme.cmd_failed_bg;
        fg = p.theme.cmd_failed_fg;
    }
    p.segments.push(Segment::new(bg, fg, format::root(p.shell)).dont_escape());
}
