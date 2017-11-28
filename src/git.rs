use std::process::Child;

#[derive(Default)]
pub struct GitOutput {
    pub local: String,
    pub ahead: u32,
    pub behind: u32,
    pub staged: u32,
    pub notstaged: u32,
    pub untracked: u32,
    pub conflict: u32
}

pub fn output(git: &mut Option<Child>, git_out: &mut Option<GitOutput>) -> bool {
    if let Some(git) = git.take() {
        *git_out = match git.wait_with_output() {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
                let mut out = GitOutput::default();

                for (i, line) in stdout.split('\0').enumerate() {
                    if i == 0 {
                        if !line.starts_with("## ") { return false; }
                        let mut line = &line[3..];
                        if let Some(i) = line.find('.') {
                            out.local = line[..i].to_string();
                            line = &line[i..];
                            if let Some(i) = line.find('[') {
                                line = &line[i+1..];
                                if !line.starts_with("ahead ") { return false; }
                                let end = line.find(']');
                                if end.is_none() { return false }

                                if let Ok(num) = line[i+6..end.unwrap()].parse() {
                                    out.ahead = num;
                                }
                                if let Some(i) = line.find('[') {
                                    line = &line[i+1..];
                                    if !line.starts_with("behind ") { return false; }
                                    let end = line.find(']');
                                    if end.is_none() { return false }

                                    if let Ok(num) = line[i+7..end.unwrap()].parse() {
                                        out.behind = num;
                                    }
                                }
                            }
                        } else {
                            out.local = line.to_string();
                        }
                        continue;
                    }

                    let mut chars = line.chars();
                    let (staged, modified) = (chars.next(), chars.next());
                    if staged == Some('?') && modified == Some('?') {
                        out.untracked += 1;
                    } else if staged == Some('U') && modified == Some('U') {
                        out.conflict += 1;
                    } else {
                        if staged.is_some() && staged.unwrap() != ' ' {
                            out.staged += 1;
                        }
                        if modified.is_some() && modified.unwrap() != ' ' {
                            out.notstaged += 1;
                        }
                    }
                }

                Some(out)
            },
            Err(_) => return false
        };
    }
    git_out.is_some()
}
