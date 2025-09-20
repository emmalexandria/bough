use std::{fmt::Formatter, io::BufWriter};

use crate::{
    format::TreeFormat,
    tree::{
        icons::{Icon, IconType},
        Tree, TreeItem,
    },
};

pub struct TextOutputConfig {
    pub ansi: bool,
    pub indent_width: usize,
    pub icon_type: IconType,
}

pub struct TextOutput {
    icon_type: IconType,
    indent_width: usize,
}

impl TextOutput {
    pub const fn new(config: TextOutputConfig) -> Self {
        Self {
            icon_type: config.icon_type,
            indent_width: config.indent_width,
        }
    }

    pub fn display_item(&self, item: &TreeItem) -> String {
        let icon = item.get_icon(self.icon_type).unwrap_or("".to_string());
        format!(
            "{}{} {}",
            " ".repeat(item.depth * self.indent_width),
            icon,
            item.name,
        )
    }
}

impl TreeFormat for TextOutput {
    type Error = String;
    type Output = String;

    fn output(&self, tree: &Tree, width: usize) -> Result<Self::Output, Self::Error> {
        Ok(String::new())
    }
}
