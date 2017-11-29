pub mod segment_cwd;
pub use self::segment_cwd::*;

#[cfg(feature = "git2")] pub mod segment_git;
#[cfg(feature = "git2")] pub use self::segment_git::*;
