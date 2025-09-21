//! `bough-lib` is the crate which powers the [Bough](https://github.com/emmalexandria/bough)
//! project.

mod border;
pub mod format;
mod tree;

pub use tree::filetypes;
pub use tree::fs_tree;
pub use tree::icons;
pub use tree::{Tree, TreeItem, TreeOptions};
