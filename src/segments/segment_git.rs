#[cfg(feature = "flame")] use flame;
use {Powerline, Segment};
use git2::{self, BranchType, ObjectType, Repository, StatusOptions, StatusShow};

fn discover_if_none(git: &mut Option<Repository>) -> bool {
    #[cfg(feature = "flame")]
    let _guard = flame::start_guard("git discover");

    if git.is_none() {
        *git = Repository::discover(".").ok();
        git.is_some()
    } else { true }
}
fn statuses_if_none(git: &Repository, statuses: &mut Option<Vec<git2::Status>>) -> bool {
    #[cfg(feature = "flame")]
    let _guard = flame::start_guard("git status");

    if statuses.is_none() {
        *statuses = git.statuses(Some(
                StatusOptions::new()
                    .show(StatusShow::IndexAndWorkdir)
                    .include_untracked(true)
                    .renames_from_rewrites(true)
                    .renames_head_to_index(true)
            ))
            .ok()
            .map(|statuses|
                statuses.iter()
                .map(|entry| entry.status())
                .collect());
        statuses.is_some()
    } else { true }
}

pub fn segment_git(p: &mut Powerline) {
    #[cfg(feature = "flame")]
    let _guard = flame::start_guard("segment git");

    if !discover_if_none(&mut p.git) {
        return;
    }
    let git = p.git.as_ref().unwrap();

    #[cfg(feature = "flame")]
    flame::start("iter branches");

    let branches = git.branches(Some(BranchType::Local));
    if branches.is_err() {
        return;
    }

    let mut branch_name = None;
    let mut local    = None;
    let mut upstream = None;

    for branch in branches.unwrap() {
        if let Ok((branch, _)) = branch {
            if branch.is_head() {
                local    = branch.get().target();
                upstream = branch.upstream().ok().and_then(|b| b.get().target());

                if let Ok(name) = branch.name() {
                    if let Some(name) = name {
                        branch_name = Some(name.to_string());
                        break;
                    }
                }
            }
        }
    }

    #[cfg(feature = "flame")]
    flame::end("iter branches");

    if branch_name.is_none() {
        #[cfg(feature = "flame")]
        let _guard = flame::start_guard("search head");

        // Could be a detached head
        if let Ok(head) = git.head() {
            if let Some(target) = head.target() {
                branch_name = git.find_object(target, Some(ObjectType::Any))
                            .ok()
                            .and_then(|obj| obj.short_id().ok())
                            .and_then(|buf| buf.as_str()
                                                .map(|s| s.to_string()))
            }
        } else {
            p.segments.push(Segment::new(p.theme.git_dirty_bg, p.theme.git_dirty_fg, "Big Bang"));
            return;
        }
    }

    if !statuses_if_none(git, &mut p.git_statuses) {
        return;
    }
    let statuses = p.git_statuses.as_ref().unwrap();

    let (mut bg, mut fg) = (p.theme.git_dirty_bg, p.theme.git_dirty_fg);
    if statuses.is_empty() {
        bg = p.theme.git_clean_bg;
        fg = p.theme.git_clean_fg;
    }
    p.segments.push(Segment::new(bg, fg, branch_name.unwrap()));

    #[cfg(feature = "flame")]
    let _guard = flame::start_guard("checking remotes");

    if let Some(local) = local {
        if let Some(upstream) = upstream {
            if let Ok((ahead, behind)) = git.graph_ahead_behind(local, upstream) {
                if ahead > 0 {
                    let mut ahead = if ahead == 1 { String::new() } else { ahead.to_string() };
                    ahead.push('⬆');
                    p.segments.push(Segment::new(p.theme.git_ahead_bg, p.theme.git_ahead_fg, ahead));
                }

                if behind > 0 {
                    let mut behind = if behind == 1 { String::new() } else { behind.to_string() };
                    behind.push('⬇');
                    p.segments.push(Segment::new(p.theme.git_behind_bg, p.theme.git_behind_fg, behind));
                }
            }
        }
    }
}
pub fn segment_gitstage(p: &mut Powerline) {
    #[cfg(feature = "flame")]
    let _guard = flame::start_guard("segment gitstage");

    if !discover_if_none(&mut p.git) {
        return;
    }
    let git = p.git.as_ref().unwrap();

    if !statuses_if_none(git, &mut p.git_statuses) {
        return;
    }
    let statuses = p.git_statuses.as_ref().unwrap();

    #[cfg(feature = "flame")]
    flame::start("counting");

    let mut staged = 0;
    let mut notstaged = 0;
    let mut untracked = 0;
    let mut conflicted = 0;

    for status in statuses {
        if status.contains(git2::STATUS_INDEX_NEW)
            || status.contains(git2::STATUS_INDEX_MODIFIED)
            || status.contains(git2::STATUS_INDEX_TYPECHANGE)
            || status.contains(git2::STATUS_INDEX_RENAMED)
            || status.contains(git2::STATUS_INDEX_DELETED) {
            staged += 1;
        }
        if status.contains(git2::STATUS_WT_MODIFIED)
            || status.contains(git2::STATUS_WT_TYPECHANGE)
            || status.contains(git2::STATUS_WT_DELETED) {
            notstaged += 1;
        }
        if status.contains(git2::STATUS_WT_NEW) {
            untracked += 1;
        }
        if status.contains(git2::STATUS_CONFLICTED) {
            conflicted += 1;
        }
    }

    #[cfg(feature = "flame")]
    flame::end("counting");

    if staged > 0 {
        let mut string = if staged == 1 { String::with_capacity(1) } else { staged.to_string() };
        string.push('✔');
        p.segments.push(Segment::new(p.theme.git_staged_bg, p.theme.git_staged_fg, string));
    }
    if notstaged > 0 {
        let mut string = if notstaged == 1 { String::with_capacity(1) } else { notstaged.to_string() };
        string.push('✎');
        p.segments.push(Segment::new(p.theme.git_notstaged_bg, p.theme.git_notstaged_fg, string));
    }
    if untracked > 0 {
        let mut string = if untracked == 1 { String::with_capacity(1) } else { untracked.to_string() };
        string.push('+');
        p.segments.push(Segment::new(p.theme.git_untracked_bg, p.theme.git_untracked_fg, string));
    }
    if conflicted > 0 {
        let mut string = if conflicted == 1 { String::with_capacity(1) } else { conflicted.to_string() };
        string.push('*');
        p.segments.push(Segment::new(p.theme.git_conflicted_bg, p.theme.git_conflicted_fg, string));
    }
}
