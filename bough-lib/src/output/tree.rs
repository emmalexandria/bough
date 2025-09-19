use crate::output::{filetypes::EntryType, os_str_to_string};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Entry {
    name: String,
    description: Option<String>,
    e_type: EntryType,
    children: Vec<Entry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tree {
    root: Entry,
}
