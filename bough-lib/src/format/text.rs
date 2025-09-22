use crate::{
    file::{FileTree, FileTreeItem},
    format::TreeFormat,
};

pub enum Section {
    Icon,
    Name,
    Times,
    Contents,
    Comment,
}

pub struct Config {
    hidden: bool,
    git: bool,
}

pub struct TextFormat {
    pub sections: Vec<Section>,
}

impl Default for TextFormat {
    fn default() -> Self {
        Self {
            sections: vec![
                Section::Icon,
                Section::Name,
                Section::Comment,
                Section::Contents,
            ],
        }
    }
}

impl TreeFormat<FileTree, Config> for TextFormat {
    fn display(&self, tree: &FileTree, config: &Config) {}
}
