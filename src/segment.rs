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
    pub fn print(&self, next: Option<&Segment>) {
        print!("{}{} {} ", Fg(self.fg), Bg(self.bg), self.text);
        match next {
            Some(next) if next.bg == self.bg => print!("{}", Fg(SEPARATOR_FG)),
            Some(next) => print!("{}{}", Fg(self.bg), Bg(next.bg)),
            None       => print!("{}{}{}",Fg(self.bg), Reset(false), Reset(true))
        }
    }
}
