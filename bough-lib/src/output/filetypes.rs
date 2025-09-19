/// The type of the entry
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntryType {
    Folder,
    File(FileType),
}

/// The file type of a [File](EntryType::File)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileType {
    Text(Language),
    Binary,
    Image,
    Video,
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
