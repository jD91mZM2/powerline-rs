use std::borrow::Cow;
use crate::{Powerline, Segment, Shell};

pub fn segment_user(p: &mut Powerline) {
    let (bg, fg) = (p.theme.username_bg, p.theme.username_fg);
    #[cfg(feature = "users")]
    let mut bg = bg;
    let mut fg = fg;

    #[cfg(feature = "users")]
    let uid = users::get_current_uid();
    #[cfg(feature = "users")]
    { if uid == 0 {
        bg = p.theme.username_root_bg;
        fg = p.theme.username_root_fg;
    } }

    p.segments.push(match p.shell {
        Shell::Bare => Segment::new(
            bg,
            fg,
            {
                #[cfg(not(feature = "users"))]
                { Cow::from("error") }
                #[cfg(feature = "users")]
                { if let Some(user) = users::get_user_by_uid(uid) {
                    if let Some(name) = user.name().to_str() {
                        Cow::from(String::from(name))
                    } else {
                        Cow::from("error")
                    }
                } else {
                    Cow::from("error")
                } }
            }
        ),
        Shell::Bash => Segment::new(bg, fg, "\\u").dont_escape(),
        Shell::Zsh => Segment::new(bg, fg, "%n").dont_escape(),
    });
}
