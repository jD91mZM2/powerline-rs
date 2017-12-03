use git2::{self, BranchType, ObjectType, Repository, StatusOptions, StatusShow};
use segment::Segment;
use theme::Theme;

pub fn segment_git(segments: &mut Vec<Segment>, theme: &Theme, git: &Option<Repository>) {
    if git.is_none() {
        return;
    }
    let git = git.as_ref().unwrap();

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

    if branch_name.is_none() {
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
            segments.push(Segment::new(theme.git_dirty_bg, theme.git_dirty_fg, "Big Bang"));
            return;
        }
    }

    let statuses = git.statuses(Some(
        StatusOptions::new()
            .show(StatusShow::IndexAndWorkdir)
            .include_untracked(true)
    ));
    if statuses.is_err() {
        return;
    }

    let (mut bg, mut fg) = (theme.git_dirty_bg, theme.git_dirty_fg);
    if statuses.unwrap().is_empty() {
        bg = theme.git_clean_bg;
        fg = theme.git_clean_fg;
    }
    segments.push(Segment::new(bg, fg, branch_name.unwrap()));

    if let Some(local) = local {
        if let Some(upstream) = upstream {
            if let Ok((ahead, behind)) = git.graph_ahead_behind(local, upstream) {
                if ahead > 0 {
                    let mut ahead = if ahead == 1 { String::new() } else { ahead.to_string() };
                    ahead.push('⬆');
                    segments.push(Segment::new(theme.git_ahead_bg, theme.git_ahead_fg, ahead));
                }

                if behind > 0 {
                    let mut behind = if behind == 1 { String::new() } else { behind.to_string() };
                    behind.push('⬇');
                    segments.push(Segment::new(theme.git_behind_bg, theme.git_behind_fg, behind));
                }
            }
        }
    }
}

pub fn segment_gitstage(segments: &mut Vec<Segment>, theme: &Theme, git: &Option<Repository>) {
    if git.is_none() {
        return;
    }
    let git = git.as_ref().unwrap();

    let statuses = git.statuses(Some(
        StatusOptions::new()
            .show(StatusShow::IndexAndWorkdir)
            .include_untracked(true)
            .renames_from_rewrites(true)
            .renames_head_to_index(true)
    ));
    if statuses.is_err() {
        return;
    }

    let mut staged = 0;
    let mut notstaged = 0;
    let mut untracked = 0;
    let mut conflicted = 0;

    for status in statuses.unwrap().iter() {
        let status = status.status();
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

    if staged > 0 {
        let mut string = if staged == 1 { String::with_capacity(1) } else { staged.to_string() };
        string.push('✔');
        segments.push(Segment::new(theme.git_staged_bg, theme.git_staged_fg, string));
    }
    if notstaged > 0 {
        let mut string = if notstaged == 1 { String::with_capacity(1) } else { notstaged.to_string() };
        string.push('✎');
        segments.push(Segment::new(theme.git_notstaged_bg, theme.git_notstaged_fg, string));
    }
    if untracked > 0 {
        let mut string = if untracked == 1 { String::with_capacity(1) } else { untracked.to_string() };
        string.push('+');
        segments.push(Segment::new(theme.git_untracked_bg, theme.git_untracked_fg, string));
    }
    if conflicted > 0 {
        let mut string = if conflicted == 1 { String::with_capacity(1) } else { conflicted.to_string() };
        string.push('*');
        segments.push(Segment::new(theme.git_conflicted_bg, theme.git_conflicted_fg, string));
    }
}
