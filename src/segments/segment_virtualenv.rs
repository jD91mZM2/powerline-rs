use std::{env, path};
use crate::{Powerline, Segment};

pub fn segment_virtualenv(p: &mut Powerline) {
    if let Ok(Some(virtual_env_name)) = env::var("VIRTUAL_ENV")
        .or_else(|_| env::var("CONDA_ENV_PATH"))
        .or_else(|_| env::var("CONDA_DEFAULT_ENV"))
        .map(|env_path| {
            path::Path::new(&env_path)
                .file_name()
                .and_then(|env_name| Some(env_name.to_string_lossy().into_owned()))
        }) {
        p.segments.push(Segment::new(
            p.theme.virtual_env_bg,
            p.theme.virtual_env_fg,
            virtual_env_name,
        ));
    }
}
