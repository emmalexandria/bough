use std::{error::Error as StdError, fmt::Display};

/// A trait for valid IDs to be used within the tree. IDs must be convertible to and from usize and
/// comparable. This is implemented for most numeric types
pub trait TreeId: Eq + From<usize> + Into<usize> {
    /// The ID value for an "invalid" ID
    ///
    /// This is used in cases where we need an ID to construct a type,
    /// but won't have the correct ID until a bit later
    const INVALID: Self;

    /// Returns if the ID is invalid
    fn is_invalid(&self) -> bool {
        *self == Self::INVALID
    }
}

impl TreeId for usize {
    const INVALID: Self = Self::MAX;
}

/// The trait for an item in an [ArenaTree].
///
/// This trait tries to put as few requirements on types as possible, but it inherently requires
/// storing the children and parent of the item in whatever type you chose to be your tree's ID.
pub trait TreeItem<I> {
    /// Get the children of the item.
    fn children(&self) -> &Vec<I>;

    /// Get the parent of the item.
    fn parent(&self) -> Option<I>;

    /// Set the parent of the item.
    fn set_parent(&mut self, parent: I);

    /// Add a child to the items children.
    fn add_child(&mut self, child: I);
}

#[derive(Debug)]
pub enum ErrorKind {
    DoesNotExist,
    NeedsParent,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}

impl Error {
    pub fn new<S: ToString>(kind: ErrorKind, message: S) -> Self {
        Self {
            kind,
            message: message.to_string(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl StdError for Error {}

/// ArenaTree is a generic trait based implementation of a tree structure.
///
/// Although bough-lib only provides a tree for files itself, this is provided to make it easier to
/// create your own tree types to be displayed. The advantage of arena tree is that, basically, it
/// plays nicer with Rust's borrow checker (no raw pointers).
///
/// To create a displayable tree, see the [Tree] trait.
pub struct ArenaTree<T, I>
where
    I: TreeId + Copy,
    T: TreeItem<I> + Clone + Eq,
{
    /// The root ID of the tree
    pub root: I,

    /// An "arena" for nodes of the tree
    pub nodes: Vec<Option<T>>,
    /// Free but allocated spaces in the nodes vec
    pub free: Vec<I>,
}

impl<T, I> ArenaTree<T, I>
where
    I: TreeId + Copy,
    T: TreeItem<I> + Clone + Eq,
{
    /// Create a new tree with the given root.
    ///
    /// Capacity represents the pre-allocated size of the "arena" ([Vec]). If capacity is not passed, it will default to 1024.
    #[inline]
    pub fn new(root: T, capacity: Option<usize>) -> Self {
        let capacity = capacity.unwrap_or(1024);
        let mut ret = Self {
            root: 0.into(),
            nodes: Vec::new(),
            free: Vec::with_capacity(capacity),
        };

        ret.nodes[ret.root.into()] = Some(root);

        ret
    }

    /// Create a new empty tree without a root
    #[inline]
    pub const fn empty(capacity: usize) -> Self {
        Self {
            root: I::INVALID,
            free: Vec::new(),
            nodes: Vec::new(),
        }
    }

    /// Set the root of the tree. Intended to be used with [empty](ArenaTree::empty).
    #[inline]
    pub fn root(mut self, root: T) -> Self {
        self.nodes.clear();
        self.free.clear();
        self.root = self.allocate_id();
        self.nodes[self.root.into()] = Some(root);
        self
    }

    /// Get a node by ID
    #[inline]
    pub fn get_node(&self, id: I) -> Option<&T> {
        self.nodes.get(id.into())?.as_ref()
    }

    /// Get a mutable reference to a node by ID
    #[inline]
    pub fn get_node_mut(&mut self, id: I) -> Option<&mut T> {
        self.nodes.get_mut(id.into())?.as_mut()
    }

    /// Insert a node into the tree. This node must have a parent ID.
    #[inline]
    pub fn insert_node(&mut self, child: T) -> Result<I, Error> {
        let id = self.allocate_id();
        if child.parent().is_none() {
            return Err(Error::new(
                ErrorKind::NeedsParent,
                "Inserting a node requires it has a parent",
            ));
        }

        let parent = self.get_node_mut(child.parent().unwrap());
        if parent.is_none() {
            return Err(Error::new(
                ErrorKind::DoesNotExist,
                "Could not create node as parent does not exist",
            ));
        }

        parent.unwrap().add_child(id);
        self.nodes[id.into()] = Some(child);

        Ok(id)
    }

    /// Copy a node from one part of the tree to another
    #[inline]
    pub fn copy_node(&mut self, node: I, new_parent: I) -> Result<I, Error> {
        let node = self.get_node(node).cloned();

        if let Some(mut n) = node {
            n.set_parent(new_parent);
            return self.insert_node(n);
        }

        Err(Error::new(
            ErrorKind::DoesNotExist,
            "Source node does not exist to copy",
        ))
    }

    /// Find a node by value
    #[inline]
    pub fn find_node(&self, node: &T) -> Option<I> {
        self.nodes
            .iter()
            .position(|n| n.as_ref() == Some(node))
            .map(Into::into)
    }

    fn allocate_id(&mut self) -> I {
        if let Some(id) = self.free.pop() {
            id
        } else {
            let id = self.nodes.len().into();
            self.nodes.push(None);
            id
        }
    }

    /// Get the length of the tree in terms of the number of nodes
    #[inline]
    pub fn len(&self) -> usize {
        self.assigned_nodes().len()
    }

    /// Calculate the depth of a node by following its parents.
    ///
    /// It is recommended to maintain a depth field inside your TreeItem over using
    /// this function directly. This function is intended for scenarios such as updating your
    /// internal depth field when copying or moving a node.
    ///
    /// This function is guaranteed to return [Some] if the passed ID is valid, but
    /// note that as soon as it encounters a [None] parent it will return that depth.
    /// If you have managed to create an isolated subtree, this function will only
    /// return the depth within that subtree and not an error.
    #[inline]
    pub fn node_depth(&self, id: I) -> Option<usize> {
        let mut curr_node = self.get_node(id)?.parent();
        let mut depth = 0;

        while let Some(p) = curr_node {
            depth += 1;
            curr_node = self.get_node(p)?.parent();
        }

        Some(depth)
    }

    /// Get a vector of pointers to the nodes which are assigned (not None)
    #[inline]
    pub fn assigned_nodes(&self) -> Vec<&T> {
        self.nodes
            .iter()
            .filter(|n| n.is_some())
            .map(|n| n.as_ref().unwrap())
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::{ArenaTree, TreeItem};

    #[derive(Clone, PartialEq, Eq)]
    struct BasicNode {
        pub parent: Option<usize>,
        pub children: Vec<usize>,
    }

    impl TreeItem<usize> for BasicNode {
        fn children(&self) -> &Vec<usize> {
            &self.children
        }

        fn parent(&self) -> Option<usize> {
            self.parent
        }

        fn set_parent(&mut self, parent: usize) {
            self.parent = Some(parent);
        }

        fn add_child(&mut self, child: usize) {
            self.children.push(child);
        }
    }

    fn create_test_node(parent: usize) -> BasicNode {
        BasicNode {
            parent: Some(parent),
            children: Vec::new(),
        }
    }

    fn create_test_tree() -> ArenaTree<BasicNode, usize> {
        let root = BasicNode {
            parent: None,
            children: Vec::new(),
        };

        let mut tree = ArenaTree::empty(10).root(root);

        let root_children: Vec<BasicNode> = vec![
            BasicNode {
                parent: Some(tree.root),
                children: Vec::new(),
            },
            BasicNode {
                parent: Some(tree.root),
                children: Vec::new(),
            },
        ];

        for child in root_children {
            tree.insert_node(child);
        }

        tree
    }

    #[test]
    fn test_node_depth() {
        let root = BasicNode {
            children: Vec::new(),
            parent: None,
        };

        let tree = create_test_tree();

        assert_eq!(tree.node_depth(1), Some(1));
    }
}
