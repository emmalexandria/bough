//! `bough-lib` is the crate which powers the [Bough](https://github.com/emmalexandria/bough)
//! project.

mod border;

mod tree;

pub use tree::file;
pub use tree::{ArenaTree, TreeId, TreeItem};
