use std::os::raw::{c_char, c_int};
use {Powerline, Segment};

const W_OK: c_int = 2;

extern "C" {
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
}

pub fn segment_perms(p: &mut Powerline) {
    if unsafe { access(".\0".as_ptr() as *const c_char, W_OK) } != 0 {
        p.segments.push(Segment::new(p.theme.ro_bg, p.theme.ro_fg, p.theme.ro_char.to_string()));
    }
}
