use std::fmt;

pub const HOME_BG: u8 = 31;
pub const HOME_FG: u8 = 15;

pub const PATH_BG: u8 = 237;
pub const PATH_FG: u8 = 250;
pub const CWD_FG:  u8 = 254;

pub struct Fg(pub u8);
impl fmt::Display for Fg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1b[38;5;{}m", self.0)
    }
}

pub struct Bg(pub u8);
impl fmt::Display for Bg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1b[48;5;{}m", self.0)
    }
}

pub struct Reset(pub bool);
impl fmt::Display for Reset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let reset = if self.0 { "39" } else { "49" };
        write!(f, "\x1b[{}m", reset)
    }
}
