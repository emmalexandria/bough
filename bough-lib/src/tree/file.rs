//! Implements [FileTree], a tree type for representing the file system.
//!
//! Internally, [FileTree] uses the [ArenaTree] generic.

use std::fmt::Display;
use std::fs::read_dir;
use std::io;
use std::path::{Path, PathBuf};

use crate::tree::tree;
use crate::{tree::os_str_to_string, ArenaTree, TreeItem};

/// The type of the file
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum FileType {
    /// Directory
    Directory,
    /// File
    File,
}

impl TryFrom<&Path> for FileType {
    type Error = io::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let metadata = path.metadata()?;

        match metadata.is_dir() {
            true => Ok(Self::Directory),
            false => Ok(Self::File),
        }
    }
}

type Id = usize;

/// An implementation of [TreeItem] for file trees.
#[derive(Clone, PartialEq, Eq)]
pub struct FileTreeItem {
    children: Vec<Id>,
    parent: Option<Id>,

    /// The full path of the item
    pub path: PathBuf,
    /// The name of the item including extension
    pub name: String,
    /// The extension of the item
    pub ext: Option<String>,
    /// The [FileType] of the item
    pub file_type: FileType,
}

impl FileTreeItem {
    /// Create a new file tree item from a given path and parent ID
    pub fn from_path<P: AsRef<Path>>(path: P, parent: Id) -> io::Result<Self> {
        let path = path.as_ref();
        let name = path.file_name().map(os_str_to_string).unwrap_or_default();
        let file_type = path.try_into()?;

        let ret = Self {
            parent: Some(parent),
            children: Vec::new(),

            path: path.into(),
            name,
            ext: path.extension().map(os_str_to_string),
            file_type,
        };

        Ok(ret)
    }
}

impl std::fmt::Debug for FileTreeItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let end = match self.file_type {
            FileType::Directory => "(dir)",
            FileType::File => "",
        };
        write!(f, "{} ({:?}) {}", self.name, self.path, end)
    }
}

impl TreeItem<Id> for FileTreeItem {
    fn children(&self) -> &Vec<Id> {
        &self.children
    }

    fn parent(&self) -> Option<Id> {
        self.parent
    }

    fn set_parent(&mut self, parent: Id) {
        self.parent = Some(parent);
    }

    fn add_child(&mut self, child: Id) {
        self.children.push(child)
    }
}

/// An implementation of a file tree using [ArenaTree]
pub struct FileTree {
    tree: ArenaTree<FileTreeItem, Id>,
    root_path: PathBuf,
}

impl FileTree {
    /// Create an empty file tree starting at the path. Please call build to actually build the
    /// file tree.
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        let metadata = path.metadata()?;

        if !metadata.is_dir() {
            return Err(io::Error::new(
                std::io::ErrorKind::NotADirectory,
                "Root path for file tree must be a directory.",
            ));
        }

        let tree = ArenaTree::empty(1024);

        Ok(Self {
            tree,
            root_path: path.into(),
        })
    }

    /// Build the file tree
    #[must_use = "moves the value of self and returns the modified value"]
    pub fn build(mut self) -> io::Result<Self> {
        let root = self.root_path.clone().try_into()?;

        self.tree = self.tree.root(root);

        self.build_from_directory(self.root_path.clone(), self.tree.root)?;

        Ok(self)
    }

    fn build_from_directory<P: AsRef<Path>>(&mut self, path: P, parent: Id) -> io::Result<()> {
        let path = path.as_ref();
        let entries = read_dir(path)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            let metadata = entry.metadata()?;

            let name = entry.file_name();
            let ext = path.extension();

            let node: FileTreeItem = path.clone().try_into()?;
            let id = self.tree.insert_node(node).map_err(|e| match e.kind {
                tree::ErrorKind::NeedsParent => {
                    io::Error::new(io::ErrorKind::NotFound, "Parent not found")
                }
                _ => io::Error::new(io::ErrorKind::Other, "Unknown error"),
            })?;

            if metadata.is_dir() {
                self.build_from_directory(path, id)?;
            }
        }

        Ok(())
    }
}

impl std::fmt::Debug for FileTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "File tree contains {} items", self.tree.len())?;
        writeln!(f, "Debug tree view")?;
        writeln!(f, "---------------")?;
        let stack: Vec<Id> = Vec::new();
        if let Some(r) = self.tree.get_node(self.tree.root) {
        } else {
            writeln!(f, "No root node!")?;
        }
        Ok(())
    }
}
