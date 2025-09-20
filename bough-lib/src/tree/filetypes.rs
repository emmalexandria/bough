use crate::tree::icons::{Icon, IconType};

/// The type of the entry
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemType {
    Directory(bool),
    File(FileType),
}

impl Icon for ItemType {
    fn get_icon(&self, icon_type: IconType) -> Option<String> {
        match self {
            Self::Directory(b) => match icon_type {
                IconType::None => None,
                IconType::Nerd => match b {
                    true => Some("󰝰".to_string()),
                    false => Some("󰉋".to_string()),
                },
                IconType::Unicode => Some("🗀".to_string()),
                IconType::Emoji => Some(if *b {
                    "📁".to_string()
                } else {
                    "📂".to_string()
                }),
            },
            Self::File(f_type) => f_type.get_icon(icon_type),
        }
    }
}

/// The file type of a [File](EntryType::File)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileType {
    Text(Language),
    Binary,
    Image,
    Video,
}

impl Icon for FileType {
    fn get_icon(&self, icon_type: IconType) -> Option<String> {
        match self {
            Self::Text(lang) => lang.get_nerd_icon(icon_type),
            Self::Binary => Some("".to_string()),
            Self::Video => Some("󰈫".to_string()),
            Self::Image => Some("󰈟".to_string()),
        }
    }
}

/// The language of [Text](FileType::Text)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Unknown,
    PlainText,
    Markdown,
    Toml,
    Json,
    Yaml,
    Rust,
    C,
    Cpp,
}

impl Language {
    pub fn get_nerd_icon(&self, icon_type: IconType) -> Option<String> {
        match self {
            Language::Unknown => todo!(),
            Language::PlainText => todo!(),
            Language::Markdown => todo!(),
            Language::Toml => todo!(),
            Language::Json => todo!(),
            Language::Yaml => todo!(),
            Language::Rust => todo!(),
            Language::C => todo!(),
            Language::Cpp => todo!(),
        }
    }
}
