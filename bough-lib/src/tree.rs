use std::ffi::OsString;

/// Implements the [FileTree](file::FileTree) type and associated types
pub mod file;
/// Implements the [ArenaTree] type and associated types.
pub mod tree;

pub use tree::{ArenaTree, TreeId, TreeItem};

pub fn os_str_to_string<S: Into<OsString>>(s: S) -> String {
    s.into().to_string_lossy().to_string()
}
