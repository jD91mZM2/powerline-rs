use Shell;
use format::*;

pub struct Segment {
    bg: u8,
    fg: u8,
    text: String
}
impl Segment {
    pub fn new<S: Into<String>>(bg: u8, fg: u8, text: S) -> Self {
        Segment {
            bg:    bg,
            fg:    fg,
            text:  text.into(),
        }
    }
    pub fn print(&self, next: Option<&Segment>, shell: Shell) {
        print!("{}{} {} ", Fg(shell, self.fg), Bg(shell, self.bg), self.text);
        match next {
            Some(next) if next.bg == self.bg => print!("{}", Fg(shell, SEPARATOR_FG)),
            Some(next) => print!("{}{}", Fg(shell, self.bg), Bg(shell, next.bg)),
            None       => print!("{}{}{}",Fg(shell, self.bg), Reset(shell, false), Reset(shell, true))
        }
    }
}
