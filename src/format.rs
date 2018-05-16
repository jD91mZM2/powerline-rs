use Shell;
use std::fmt;

pub struct Fg(pub Shell, pub u8);
impl fmt::Display for Fg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Shell::Bare => write!(f, "\x1b[38;5;{}m", self.1),
            Shell::Bash => write!(f, "\\[\\e[38;5;{}m\\]", self.1),
            Shell::Ion  => write!(f, "${{c::{}}}", self.1),
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
            Shell::Ion  => write!(f, "${{c::{}bg}}", self.1),
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
            Shell::Ion  => write!(f, "${{c::reset}}"),
            Shell::Zsh  => write!(f, "%{{\x1b[{}9m%}}", reset)
        }
    }
}

pub fn root(shell: Shell) -> &'static str {
    match shell {
        Shell::Bare => "$",
        Shell::Bash => "\\$",
        Shell::Ion  => "\\$",
        Shell::Zsh  => "%#"
    }
}
pub fn escape(shell: Shell, string: &mut String) {
    if shell == Shell::Bare {
        return;
    }
    let mut output = String::with_capacity(string.len());
    for c in string.chars() {
        match shell {
            Shell::Bash => match c {
                '\\' => output.push_str("\\\\"),
                '$'  => output.push_str("\\$"),
                '"'  => output.push_str("\\\""),
                c    => output.push(c)
            },
            Shell::Ion => match c {
                '\\' => output.push_str("\\\\"),
                '$'  => output.push_str("\\$"),
                '"'  => output.push_str("\\\""),
                c    => output.push(c)
            },

            Shell::Zsh => match c {
                '%' => output.push_str("%%"),
                ')' => output.push_str("%)"),
                c   => output.push(c)
            },
            Shell::Bare => unreachable!()
        }
    }
    *string = output;
}
