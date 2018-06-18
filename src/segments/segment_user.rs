use std::borrow::Cow;
use users;
use {Powerline, Segment, Shell};

pub fn segment_user(p: &mut Powerline) {
    let (mut bg, fg) = (p.theme.username_bg, p.theme.username_fg);

    let uid = users::get_current_uid();
    if uid == 0 {
        bg = p.theme.username_root_bg;
    }

    p.segments.push(match p.shell {
        Shell::Bare => Segment::new(
            bg,
            fg,
            if let Some(user) = users::get_user_by_uid(uid) {
                Cow::from(String::from(user.name()))
            } else {
                Cow::from("error")
            },
        ),
        Shell::Bash => Segment::new(bg, fg, "\\u").dont_escape(),
        Shell::Zsh => Segment::new(bg, fg, "%n").dont_escape(),
    });
}
