use std::{borrow::Cow, collections::HashMap};

use crate::output::filetypes::FileType;

pub struct HTMLClasses<'a> {
    directory: Option<Cow<'a, str>>,
    file: Option<Cow<'a, str>>,
    file_types: HashMap<FileType, Cow<'a, str>>,
}

impl<'a> HTMLClasses<'a> {
    pub fn new() -> Self {
        Self {
            directory: None,
            file: None,
            file_types: HashMap::new(),
        }
    }

    pub fn directory<S: Into<Cow<'a, str>>>(mut self, class: S) -> Self {
        self.directory = Some(class.into());
        self
    }

    pub fn file<S: Into<Cow<'a, str>>>(mut self, class: S) -> Self {
        self.file = Some(class.into());
        self
    }

    pub fn add_file_class<S: Into<Cow<'a, str>>>(mut self, f_type: FileType, class: S) -> Self {
        self.file_types.insert(f_type, class.into());
        self
    }
}

pub struct HTMLSettings<'a> {
    default_css: bool,
    classes: HTMLClasses<'a>,
}
