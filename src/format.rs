use std::fmt;

pub const SEPARATOR_FG: u8 = 244;

pub const HOME_BG: u8 = 31;
pub const HOME_FG: u8 = 15;

pub const PATH_BG: u8 = 237;
pub const PATH_FG: u8 = 250;
pub const CWD_FG:  u8 = 254;

pub const REPO_CLEAN_BG: u8 = 148;
pub const REPO_CLEAN_FG: u8 = 0;
pub const REPO_DIRTY_BG: u8 = 161;
pub const REPO_DIRTY_FG: u8 = 15;
pub const GIT_CONFLICTED_BG: u8 = 9;
pub const GIT_CONFLICTED_FG: u8 = 15;
pub const GIT_NOTSTAGED_BG: u8 = 130;
pub const GIT_NOTSTAGED_FG: u8 = 15;
pub const GIT_STAGED_BG:    u8 = 22;
pub const GIT_STAGED_FG:    u8 = 15;
pub const GIT_UNTRACKED_BG: u8 = 52;
pub const GIT_UNTRACKED_FG: u8 = 15;

pub const CMD_PASSED_BG: u8 = 236;
pub const CMD_PASSED_FG: u8 = 15;
pub const CMD_FAILED_BG: u8 = 161;
pub const CMD_FAILED_FG: u8 = 15;

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
