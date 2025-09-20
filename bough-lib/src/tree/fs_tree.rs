use std::{
    fs::{metadata, read_dir},
    io,
    path::Path,
};

use crate::{
    filetypes::{FileType, ItemType},
    tree::os_str_to_string,
};

pub type NodeId = usize;

pub const NULL_NODE: NodeId = usize::MAX;

#[derive(Debug, Clone)]
pub enum NodeType {
    Directory(Vec<NodeId>),
    File,
}

#[derive(Debug, Clone)]
pub struct FsNode {
    pub name: String,
    pub ext: Option<String>,
    pub node_type: ItemType,
    pub parent: NodeId,
    pub children: Vec<NodeId>,
}

pub struct FsTree {
    nodes: Vec<Option<FsNode>>,
    free_list: Vec<NodeId>,

    root: NodeId,
}

impl FsTree {
    pub fn new<P: AsRef<Path>>(root_name: P, capacity: Option<usize>) -> Self {
        let capacity = capacity.unwrap_or(1024);
        let path = root_name.as_ref();
        let root_name = path
            .file_name()
            .unwrap_or_else(|| path.as_os_str())
            .to_string_lossy()
            .to_string();
        let root_ext = path.extension().map(os_str_to_string);
        let mut tree = Self {
            nodes: Vec::with_capacity(128),
            free_list: Vec::new(),
            root: 0,
        };

        let root_node = FsNode {
            name: root_name,
            ext: root_ext,
            node_type: ItemType::Directory(false),
            parent: NULL_NODE,
            children: Vec::new(),
        };

        tree.nodes.push(Some(root_node));
        tree
    }

    pub fn build<P: AsRef<Path>>(path: P, capacity: Option<usize>) -> std::io::Result<Self> {
        let path = path.as_ref();
        let metadata = metadata(path)?;

        if !metadata.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Root of tree must be a directory",
            ));
        }

        let root_name = path
            .file_name()
            .unwrap_or_else(|| path.as_os_str())
            .to_string_lossy()
            .to_string();

        let mut tree = FsTree::new(root_name, capacity);

        tree.build_from_directory(path, tree.root)?;

        Ok(tree)
    }

    fn build_from_directory<P: AsRef<Path>>(
        &mut self,
        path: P,
        parent_id: NodeId,
    ) -> std::io::Result<()> {
        let path = path.as_ref();

        let entries = read_dir(path)?;
        let mut children = Vec::new();

        for entry in entries {
            let entry = entry?;
            let e_path = entry.path();
            let metadata = entry.metadata()?;

            let name = entry.file_name().to_string_lossy().to_string();
            let ext = e_path.extension().map(os_str_to_string);
            let node_id = self.allocate_node_id();

            if metadata.is_dir() {
                let node = FsNode {
                    name,
                    ext,
                    node_type: ItemType::Directory(false),
                    parent: parent_id,
                    children: Vec::new(),
                };

                self.nodes[node_id] = Some(node);
                children.push((node_id, e_path, true));
            } else if metadata.is_file() {
                let node = FsNode {
                    name,
                    ext,
                    node_type: ItemType::File(FileType::Binary),
                    parent: parent_id,
                    children: Vec::new(),
                };

                self.nodes[node_id] = Some(node);
                children.push((node_id, e_path, false));
            }
        }

        if let Some(parent) = self.get_node_mut(parent_id) {
            if let ItemType::Directory(_) = parent.node_type {
                for (c_id, _, _) in &children {
                    parent.children.push(*c_id);
                }
            }
        }

        for (child_id, entry_path, is_dir) in children {
            if is_dir {
                self.build_from_directory(&entry_path, child_id)?;
            }
        }

        Ok(())
    }

    fn allocate_node_id(&mut self) -> NodeId {
        if let Some(id) = self.free_list.pop() {
            id
        } else {
            let id = self.nodes.len() as NodeId;
            self.nodes.push(None);
            id
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    #[inline]
    pub fn get_node(&self, id: NodeId) -> Option<&FsNode> {
        self.nodes.get(id)?.as_ref()
    }

    #[inline]
    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut FsNode> {
        self.nodes.get_mut(id)?.as_mut()
    }
}
