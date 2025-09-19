use std::ffi::{OsStr, OsString};

pub mod display;
pub mod filetypes;
pub mod icons;
pub mod tree;

pub fn os_str_to_string<S: Into<OsString>>(s: S) -> String {
    s.into().to_string_lossy().to_string()
}
