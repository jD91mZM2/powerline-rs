use std::env;
use {Powerline, Segment};

pub fn segment_nix(p: &mut Powerline) {
    // TODO: Generalize this to any environment variable?
    if let Ok(val) = env::var("IN_NIX_SHELL") {
        p.segments.push(Segment::new(
            p.theme.nixshell_bg,
            p.theme.nixshell_fg,
            val
        ));
    }
}
