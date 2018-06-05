use std::env;
use {Powerline, Segment, Shell};

pub fn segment_host(p: &mut Powerline) {
    let (bg, fg) = (p.theme.hostname_bg, p.theme.hostname_fg);

    if p.shell == Shell::Bare {
        // We don't want to dont_escape() here
        p.segments.push(
            Segment::new(bg, fg, env::var("HOSTNAME")
                .unwrap_or_else(|_| String::from("error")))
        );
        return;
    }

    p.segments.push(Segment::new(bg, fg, match p.shell {
        Shell::Bare => unreachable!(),
        Shell::Bash => "\\h",
        Shell::Zsh  => "%m"
    }).dont_escape());
}
