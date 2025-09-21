use std::{
    ffi::{OsStr, OsString},
    path::PathBuf,
};

pub mod file;
pub mod filetypes;
pub mod icons;

use icons::IconType;

use crate::tree::icons::Icon;

pub fn os_str_to_string<S: Into<OsString>>(s: S) -> String {
    s.into().to_string_lossy().to_string()
}

/// Tree is a generic type for tree formats to be displayed by bough-lib.
pub trait Tree {
    type Item: TreeItem;
    type Id;

    fn get_children(&self, id: Self::Id) -> Vec<Self::Item>;
}

pub trait TreeItem {
    fn get_icon(&self) -> &str;
}
