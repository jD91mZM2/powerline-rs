use {Powerline, Segment, Shell};

pub fn segment_jobs(p: &mut Powerline) {
    p.segments.push(match p.shell {
        Shell::Bare => return,
        Shell::Bash =>
            Segment::new(p.theme.jobs_bg, p.theme.jobs_fg, "\\j")
                .with_before(r#"$(test -n "$(jobs -p)" && echo -n ""#)
                .with_after(r#"")"#),
        Shell::Zsh =>
            Segment::new(p.theme.jobs_bg, p.theme.jobs_fg, "%j")
                .with_before("%(1j.")
                .with_after(".)"),
    }.into_conditional().dont_escape());
}
