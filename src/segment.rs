use Shell;
use format::*;
use theme::Theme;

pub struct Segment {
    bg: u8,
    fg: u8,

    before: &'static str,
    after: &'static str,
    conditional: bool,

    escaped: bool,
    text: String
}
impl Segment {
    pub fn new<S: Into<String>>(bg: u8, fg: u8, text: S) -> Self {
        Segment {
            bg:    bg,
            fg:    fg,

            before: "",
            after: "",
            conditional: false,

            escaped: false,
            text:  text.into()
        }
    }
    pub fn dont_escape(mut self) -> Self {
        self.escaped = true;
        self
    }
    pub fn with_before(mut self, before: &'static str) -> Self {
        self.before = before;
        self
    }
    pub fn with_after(mut self, after: &'static str) -> Self {
        self.after = after;
        self
    }
    pub fn as_conditional(mut self) -> Self {
        self.conditional = true;
        self
    }
    pub fn is_conditional(&self) -> bool {
        self.conditional
    }
    pub fn escape(&mut self, shell: Shell) {
        if self.escaped {
            return;
        }
        escape(shell, &mut self.text);
        self.escaped = true;
    }
    pub fn print(&self, next: Option<&Segment>, shell: Shell, theme: &Theme) {
        print!("{}{}{} {} ", self.before, Fg(shell, self.fg), Bg(shell, self.bg), self.text);
        match next {
            Some(next) if next.is_conditional() => {},
            Some(next) if next.bg == self.bg => print!("{}", Fg(shell, theme.separator_fg)),
            Some(next) => print!("{}{}",  Fg(shell, self.bg), Bg(shell, next.bg)),
            // Last tile resets colors
            None       => print!("{}{}{}",Fg(shell, self.bg), Reset(shell, false), Reset(shell, true))
        }
        print!("{}", self.after);
    }
}
