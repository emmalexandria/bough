use std::{
    ffi::{OsStr, OsString},
    path::PathBuf,
};

pub mod filetypes;
pub mod fs_tree;
pub mod icons;

use icons::IconType;

use crate::tree::icons::Icon;

pub fn os_str_to_string<S: Into<OsString>>(s: S) -> String {
    s.into().to_string_lossy().to_string()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TreeOptions {
    pub icons: IconType,
    pub hidden: bool,
    pub depth: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tree {
    icons: IconType,
    depth: usize,
    show_hidden: bool,
    pub root: Option<TreeItem>,
}

impl Tree {
    pub fn with_options(options: TreeOptions) -> Self {
        Self {
            icons: options.icons,
            depth: options.depth,
            show_hidden: options.hidden,
            root: None,
        }
    }

    pub fn build<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.root = None;
        self
    }

    pub const fn toggle_hidden_files(&mut self) {
        self.show_hidden = !self.show_hidden
    }
}

impl Default for Tree {
    fn default() -> Self {
        Self::with_options(TreeOptions {
            icons: IconType::Emoji,
            depth: 0,
            hidden: false,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TreeItem {
    pub name: String,
    pub depth: usize,
    pub index: usize,
    pub parent: Option<usize>,
}

impl Icon for TreeItem {
    fn get_icon(&self, icon_type: IconType) -> Option<String> {
        Some("d".to_string())
    }
}
