use Shell;
use format::*;
use std::mem;

pub struct Segment {
    bg: u8,
    fg: u8,

    escaped: bool,
    text: String
}
impl Segment {
    pub fn new<S: Into<String>>(bg: u8, fg: u8, text: S) -> Self {
        Segment {
            bg:    bg,
            fg:    fg,

            escaped: false,
            text:  text.into()
        }
    }
    pub fn dont_escape(mut self) -> Self {
        self.escaped = true;
        self
    }
    pub fn escape(&mut self, shell: Shell) {
        if self.escaped {
            return;
        }
        let text = mem::replace(&mut self.text, unsafe { mem::uninitialized() });
        mem::forget(mem::replace(&mut self.text, escape(shell, text)));
        self.escaped = true;
    }
    pub fn print(&self, next: Option<&Segment>, shell: Shell) {
        print!("{}{} {} ", Fg(shell, self.fg), Bg(shell, self.bg), self.text);
        match next {
            Some(next) if next.bg == self.bg => print!("{}", Fg(shell, SEPARATOR_FG)),
            Some(next) => print!("{}{}", Fg(shell, self.bg), Bg(shell, next.bg)),
            None       => print!("{}{}{}",Fg(shell, self.bg), Reset(shell, false), Reset(shell, true))
        }
    }
    pub fn len(&self) -> usize {
        self.text.len() + 2
    }
}
