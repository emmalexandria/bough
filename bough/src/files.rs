use std::{
    ffi::{OsStr, OsString},
    fmt::Display,
    fs::{read_dir, File},
    io::Read,
    path::PathBuf,
};

pub struct IgnoreFiles {
    pub hidden: bool,
    pub paths: Vec<IgnorePath>,
}

impl IgnoreFiles {
    pub fn check_if_ignored(&self, path: &PathBuf) -> std::io::Result<bool> {
        for ignore in &self.paths {
            if ignore.applies_to(path)? {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct IgnorePath {
    pub path: PathBuf,
    pub dir: bool,
}

impl IgnorePath {
    pub fn applies_to(&self, path: &PathBuf) -> std::io::Result<bool> {
        if path.is_dir() != self.dir {
            return Ok(false);
        }

        if path.canonicalize()? != self.path.canonicalize()? {
            return Ok(false);
        }

        Ok(true)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileTreeItem {
    pub path: PathBuf,
    pub is_dir: bool,
    pub depth: usize,
    pub name: OsString,
    pub ext: Option<OsString>,
    pub children: Vec<FileTreeItem>,
}

impl FileTreeItem {
    pub fn build(
        path: PathBuf,
        depth: usize,
        max_depth: usize,
        ignore: &IgnoreFiles,
    ) -> std::io::Result<Self> {
        let is_dir = path.is_dir();

        let mut item = Self {
            path: path.clone(),
            is_dir,
            depth,
            name: path
                .file_name()
                .map(OsStr::to_os_string)
                .unwrap_or_default(),
            ext: path.extension().map(OsStr::to_os_string),
            children: Vec::new(),
        };

        if is_dir && depth < max_depth {
            let entries = read_dir(&path)?;
            for entry_result in entries {
                let entry = entry_result?;
                let child_path = entry.path();
                let child_name = entry.file_name().to_string_lossy().to_string();
                if ignore.check_if_ignored(&child_path)? {
                    continue;
                }

                if ignore.hidden && child_name.starts_with(".") {
                    continue;
                }

                let child_item = FileTreeItem::build(child_path, depth + 1, max_depth, ignore)?;
                item.children.push(child_item);
            }
        }

        Ok(item)
    }
}

impl Display for FileTreeItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{}",
            " ".repeat(self.depth * 2),
            self.name.to_string_lossy(),
            if self.is_dir { "/" } else { "" }
        )?;

        for child in &self.children {
            write!(f, "{child}")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileTree {
    pub root: FileTreeItem,
    pub depth: usize,
}

impl FileTree {
    pub fn build(path: PathBuf, mut max_depth: usize, ignore: IgnoreFiles) -> Self {
        // 0 represents a tree which builds until there are no more files
        if max_depth == 0 {
            max_depth = usize::MAX;
        }
        let root = FileTreeItem::build(path, 0, max_depth, &ignore).unwrap();

        Self {
            root,
            depth: max_depth,
        }
    }
}

impl Display for FileTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)
    }
}

/// This function checks if a .git folder exists. If it does, it ignores `.git` and
/// also any files in the `.gitignore`. If no repository exists, the `.gitignore` is not read.
pub fn ignore_git() -> std::io::Result<Vec<IgnorePath>> {
    let git_dir = IgnorePath {
        dir: true,
        path: PathBuf::from(".git"),
    };

    if !git_dir.path.exists() {
        return Ok(vec![]);
    }

    let mut ignore_list = vec![git_dir];
    let gitignore_path = PathBuf::from(".gitignore");
    if !gitignore_path.is_dir() && gitignore_path.exists() {
        let mut gitignore_buffer = String::new();
        let mut gitignore = File::open(gitignore_path)?;

        gitignore.read_to_string(&mut gitignore_buffer)?;

        for line in gitignore_buffer.lines() {
            if line.is_empty() {
                continue;
            }
            if line.starts_with("/") {
                let trim = line.trim_start_matches("/");
                ignore_list.push(IgnorePath {
                    path: PathBuf::from(trim),
                    dir: true,
                })
            } else {
                ignore_list.push(IgnorePath {
                    path: PathBuf::from(line),
                    dir: false,
                })
            }
        }
    }

    Ok(ignore_list)
}
