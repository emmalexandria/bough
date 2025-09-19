use std::{borrow::Cow, collections::HashMap, fs::FileType};

use crate::output::{display::html::HTMLSettings, icons::IconType, tree::Tree};

mod html;
mod text;

pub enum OutputModes<'a> {
    Text,
    Markdown,
    HTML(HTMLSettings<'a>),
    ANSI,
}

pub fn display_tree(tree: &Tree, mode: OutputModes, icons: IconType) -> String {
    match mode {
        OutputModes::Text => text::display(tree),
        _ => String::new(),
    }
}
