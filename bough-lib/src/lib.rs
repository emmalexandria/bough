//! `bough-lib` is the crate which powers the [Bough](https://github.com/emmalexandria/bough)
//! project.

pub mod output;
mod tree;
pub mod format;

pub use tree::file;
pub use tree::{ArenaTree, TreeId, TreeItem};
