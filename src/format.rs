use Shell;
use std::fmt;

pub const SEPARATOR_FG: u8 = 244;

pub const HOME_BG: u8 = 31;
pub const HOME_FG: u8 = 15;
pub const PATH_BG: u8 = 237;
pub const PATH_FG: u8 = 250;
pub const CWD_FG:  u8 = 254;

pub const USERNAME_BG: u8 = 240;
pub const USERNAME_FG: u8 = 250;
pub const USERNAME_ROOT_BG: u8 = 124;
pub const HOSTNAME_BG: u8 = 238;
pub const HOSTNAME_FG: u8 = 250;

pub const JOBS_BG: u8 = 238;
pub const JOBS_FG: u8 = 39;

pub const TIME_BG: u8 = 238;
pub const TIME_FG: u8 = 250;

pub const SSH_BG: u8 = 166;
pub const SSH_FG: u8 = 254;

pub const RO_BG: u8 = 124;
pub const RO_FG: u8 = 254;

pub const GIT_CLEAN_BG: u8 = 148;
pub const GIT_CLEAN_FG: u8 = 0;
pub const GIT_DIRTY_BG: u8 = 161;
pub const GIT_DIRTY_FG: u8 = 15;
pub const GIT_AHEAD_BG:  u8 = 240;
pub const GIT_AHEAD_FG:  u8 = 250;
pub const GIT_BEHIND_BG: u8 = 240;
pub const GIT_BEHIND_FG: u8 = 250;
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

pub struct Fg(pub Shell, pub u8);
impl fmt::Display for Fg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Shell::Bare => write!(f, "\x1b[38;5;{}m", self.1),
            Shell::Bash => write!(f, "\\[\\e[38;5;{}m\\]", self.1),
            Shell::Zsh  => write!(f, "%{{\x1b[38;5;{}m%}}", self.1)
        }
    }
}

pub struct Bg(pub Shell, pub u8);
impl fmt::Display for Bg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Shell::Bare => write!(f, "\x1b[48;5;{}m", self.1),
            Shell::Bash => write!(f, "\\[\\e[48;5;{}m\\]", self.1),
            Shell::Zsh  => write!(f, "%{{\x1b[48;5;{}m%}}", self.1)
        }
    }
}

pub struct Reset(pub Shell, pub bool);
impl fmt::Display for Reset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let reset = if self.1 { "3" } else { "4" };
        match self.0 {
            Shell::Bare => write!(f, "\x1b[{}9m", reset),
            Shell::Bash => write!(f, "\\[\\e[{}9m\\]", reset),
            Shell::Zsh  => write!(f, "%{{\x1b[{}9m%}}", reset)
        }
    }
}

pub fn root(shell: Shell) -> &'static str {
    match shell {
        Shell::Bare => "$",
        Shell::Bash => "\\$",
        Shell::Zsh  => "%#"
    }
}
pub fn escape(shell: Shell, input: String) -> String {
    if shell == Shell::Bare {
        return input;
    }
    let mut output = String::with_capacity(input.len());
    for c in input.chars() {
        match shell {
            Shell::Bash => match c {
                '\\' => output.push_str("\\\\"),
                '$'  => output.push_str("\\$"),
                c    => output.push(c)
            },
            Shell::Zsh => match c {
                '%' => output.push_str("%%"),
                c   => output.push(c)
            },
            Shell::Bare => unreachable!()
        }
    }
    output
}
