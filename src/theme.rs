#[derive(Clone)]
pub struct Theme {
    pub separator_fg: u8,

    pub home_bg: u8,
    pub home_fg: u8,
    pub path_bg: u8,
    pub path_fg: u8,
    pub cwd_fg:  u8,

    pub username_bg: u8,
    pub username_fg: u8,
    pub username_root_bg: u8,
    pub hostname_bg: u8,
    pub hostname_fg: u8,

    pub jobs_bg: u8,
    pub jobs_fg: u8,

    pub time_bg: u8,
    pub time_fg: u8,

    pub ssh_bg: u8,
    pub ssh_fg: u8,

    pub ro_bg: u8,
    pub ro_fg: u8,

    pub git_clean_bg: u8,
    pub git_clean_fg: u8,
    pub git_dirty_bg: u8,
    pub git_dirty_fg: u8,
    pub git_ahead_bg:  u8,
    pub git_ahead_fg:  u8,
    pub git_behind_bg: u8,
    pub git_behind_fg: u8,
    pub git_conflicted_bg: u8,
    pub git_conflicted_fg: u8,
    pub git_notstaged_bg: u8,
    pub git_notstaged_fg: u8,
    pub git_staged_bg:    u8,
    pub git_staged_fg:    u8,
    pub git_untracked_bg: u8,
    pub git_untracked_fg: u8,

    pub cmd_passed_bg: u8,
    pub cmd_passed_fg: u8,
    pub cmd_failed_bg: u8,
    pub cmd_failed_fg: u8
}

pub const DEFAULT: Theme = Theme {
    separator_fg: 244,

    home_bg: 31,
    home_fg: 15,
    path_bg: 237,
    path_fg: 250,
    cwd_fg: 254,

    username_bg: 240,
    username_fg: 250,
    username_root_bg: 124,
    hostname_bg: 238,
    hostname_fg: 250,

    jobs_bg: 238,
    jobs_fg: 39,

    time_bg: 238,
    time_fg: 250,

    ssh_bg: 166,
    ssh_fg: 254,

    ro_bg: 124,
    ro_fg: 254,

    git_clean_bg: 148,
    git_clean_fg: 0,
    git_dirty_bg: 161,
    git_dirty_fg: 15,
    git_ahead_bg: 240,
    git_ahead_fg: 250,
    git_behind_bg: 240,
    git_behind_fg: 250,
    git_conflicted_bg: 9,
    git_conflicted_fg: 15,
    git_notstaged_bg: 130,
    git_notstaged_fg: 15,
    git_staged_bg: 22,
    git_staged_fg: 15,
    git_untracked_bg: 52,
    git_untracked_fg: 15,

    cmd_passed_bg: 236,
    cmd_passed_fg: 15,
    cmd_failed_bg: 161,
    cmd_failed_fg: 15
};

use std::error::Error as StdError;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct ErrCorrupt;

impl StdError for ErrCorrupt {
    fn description(&self) -> &'static str { "Corrupt theme file" }
}
impl fmt::Display for ErrCorrupt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.description()) }
}

pub fn load(file: &str) -> Result<Theme, Box<StdError>> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    let mut theme = DEFAULT.clone();

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.splitn(2, '=');

        let variable = parts.next().map(|inner| inner.trim()).ok_or_else(|| ErrCorrupt)?;
        let value    = parts.next().map(|inner| inner.trim()).ok_or_else(|| ErrCorrupt)?;

        let index = theme_index(&mut theme, variable).ok_or_else(|| ErrCorrupt)?;

        *index = value.parse()?;
    }

    Ok(theme)
}
fn theme_index<'a>(theme: &'a mut Theme, name: &str) -> Option<&'a mut u8> {
    match name {
        "separator_fg" => Some(&mut theme.separator_fg),

        "home_bg" => Some(&mut theme.home_bg),
        "home_fg" => Some(&mut theme.home_fg),
        "path_bg" => Some(&mut theme.path_bg),
        "path_fg" => Some(&mut theme.path_fg),
        "cwd_fg"  => Some(&mut theme.cwd_fg),

        "username_bg" => Some(&mut theme.username_bg),
        "username_fg" => Some(&mut theme.username_fg),
        "username_root_bg" => Some(&mut theme.username_root_bg),
        "hostname_bg" => Some(&mut theme.hostname_bg),
        "hostname_fg" => Some(&mut theme.hostname_fg),

        "jobs_bg" => Some(&mut theme.jobs_bg),
        "jobs_fg" => Some(&mut theme.jobs_fg),

        "time_bg" => Some(&mut theme.time_bg),
        "time_fg" => Some(&mut theme.time_fg),

        "ssh_bg" => Some(&mut theme.ssh_bg),
        "ssh_fg" => Some(&mut theme.ssh_fg),

        "ro_bg" => Some(&mut theme.ro_bg),
        "ro_fg" => Some(&mut theme.ro_fg),

        "git_clean_bg" => Some(&mut theme.git_clean_bg),
        "git_clean_fg" => Some(&mut theme.git_clean_fg),
        "git_dirty_bg" => Some(&mut theme.git_dirty_bg),
        "git_dirty_fg" => Some(&mut theme.git_dirty_fg),
        "git_ahead_bg" => Some(&mut theme.git_ahead_bg),
        "git_ahead_fg" => Some(&mut theme.git_ahead_fg),
        "git_behind_bg" => Some(&mut theme.git_behind_bg),
        "git_behind_fg" => Some(&mut theme.git_behind_fg),
        "git_conflicted_bg" => Some(&mut theme.git_conflicted_bg),
        "git_conflicted_fg" => Some(&mut theme.git_conflicted_fg),
        "git_notstaged_bg" => Some(&mut theme.git_notstaged_bg),
        "git_notstaged_fg" => Some(&mut theme.git_notstaged_fg),
        "git_staged_bg" => Some(&mut theme.git_staged_bg),
        "git_staged_fg" => Some(&mut theme.git_staged_fg),
        "git_untracked_bg" => Some(&mut theme.git_untracked_bg),
        "git_untracked_fg" => Some(&mut theme.git_untracked_fg),

        "cmd_passed_bg" => Some(&mut theme.cmd_passed_bg),
        "cmd_passed_fg" => Some(&mut theme.cmd_passed_fg),
        "cmd_failed_bg" => Some(&mut theme.cmd_failed_bg),
        "cmd_failed_fg" => Some(&mut theme.cmd_failed_fg),

        _ => None
    }
}
