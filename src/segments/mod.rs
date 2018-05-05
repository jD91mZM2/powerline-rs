pub mod segment_cwd;
pub mod segment_ps;

pub use self::segment_cwd::*;
pub use self::segment_ps::*;

#[cfg(feature = "git2")] pub mod segment_git;
#[cfg(feature = "git2")] pub use self::segment_git::*;
