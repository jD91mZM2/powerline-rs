pub mod segment_cwd;
pub mod segment_host;
pub mod segment_jobs;
pub mod segment_nix;
pub mod segment_perms;
pub mod segment_ps;
pub mod segment_root;
pub mod segment_ssh;
pub mod segment_time;
pub mod segment_user;
pub mod segment_virtualenv;
pub mod segment_linebreak;

pub use self::segment_cwd::*;
pub use self::segment_host::*;
pub use self::segment_jobs::*;
pub use self::segment_nix::*;
pub use self::segment_perms::*;
pub use self::segment_ps::*;
pub use self::segment_root::*;
pub use self::segment_ssh::*;
pub use self::segment_time::*;
pub use self::segment_user::*;
pub use self::segment_virtualenv::*;
pub use self::segment_linebreak::*;

#[cfg(feature = "git2")] pub mod segment_git;
#[cfg(feature = "git2")] pub use self::segment_git::*;


use crate::Shell;
use crate::format::*;
use std::borrow::Cow;
use crate::theme::Theme;

pub struct Segment {
    bg: u8,
    fg: u8,

    before: &'static str,
    after: &'static str,
    conditional: bool,
    no_space_after: bool,

    escaped: bool,
    text: Cow<'static, str>
}
impl Segment {
    pub fn new<S>(bg: u8, fg: u8, text: S) -> Self
        where S: Into<Cow<'static, str>>
    {
        Segment {
            bg,
            fg,

            before: "",
            after: "",
            conditional: false,
            no_space_after: false,

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
    pub fn into_conditional(mut self) -> Self {
        self.conditional = true;
        self
    }
    pub fn is_conditional(&self) -> bool {
        self.conditional
    }
    pub fn with_no_space_after(mut self) -> Self {
        self.no_space_after = true;
        self
    }
    pub fn escape(&mut self, shell: Shell) {
        if self.escaped {
            return;
        }
        escape(shell, self.text.to_mut());
        self.escaped = true;
    }
    pub fn print(&self, next: Option<&Segment>, shell: Shell, theme: &Theme) {
        print!("{}{}{} {}", self.before, Fg(shell, self.fg), Bg(shell, self.bg), self.text);

        if !self.no_space_after {
            print!(" ")
        }
        match next {
            Some(next) if next.is_conditional() => {},
            Some(next) if next.bg == self.bg => print!("{}", Fg(shell, theme.separator_fg)),
            Some(next) if self.bg == 0 => print!("{}{}",  Fg(shell, next.bg), Bg(shell, next.bg)),
            Some(next) => print!("{}{}",  Fg(shell, self.bg), Bg(shell, next.bg)),
            // Last tile resets colors
            None       => print!("{}{}{}",Fg(shell, self.bg), Reset(shell, false), Reset(shell, true))
        }
        print!("{}", self.after);
    }
    pub fn print_rtl(&self, next: Option<&Segment>, shell: Shell, theme: &Theme) {
        // Here, next is going leftwards - see how this func is called in main.rs .
        print!("{}", self.after);
        match next {
            Some(next) if next.is_conditional() => {},
            Some(next) if next.bg == self.bg =>
                print!("{}{}", Fg(shell, theme.separator_fg), Bg(shell, self.bg)),
            Some(next) => print!("{}{}",  Fg(shell, self.bg), Bg(shell, next.bg)),
            None       => print!("{}", Fg(shell, self.bg))
        }
        print!("{}{} {}", Fg(shell, self.fg), Bg(shell, self.bg), self.text);

        if !self.no_space_after {
            print!(" ")
        }
        print!("{}{}{}", Reset(shell, false), Reset(shell, true), self.before);
    }
}
