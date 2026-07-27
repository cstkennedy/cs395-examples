#[derive(Clone, Copy, Debug)]
pub enum NodeType {
    Null,
    Root,
    Internal,
    Leaf,
}

#[derive(Clone, Debug)]
pub(crate) struct Node {
    type_: NodeType,
    depth: usize,
}

impl Node {
    fn new_root() -> Self {
        Self {
            type_: NodeType::Root,
            depth: 0,
        }
    }

    fn new_null(depth: usize) -> Self {
        Self {
            type_: NodeType::Null,
            depth,
        }
    }

    fn new_leaf(depth: usize) -> Self {
        Self {
            type_: NodeType::Leaf,
            depth,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct TreeLevel<const CHILDREN_PER_NODE: usize> {
    depth: usize,
    pub nodes: Vec<Node>,

    _marker: std::marker::PhantomData<[(); CHILDREN_PER_NODE]>
}

// Todo: Add proper errors (e.g., level checking)
impl<const CHILDREN_PER_NODE: usize> TreeLevel<CHILDREN_PER_NODE> {
    pub(crate) fn add(&mut self, node_type: NodeType) {
        match node_type {
            NodeType::Root => self.nodes.push(Node::new_root()),
            _ => todo!(),
        }
    }

    // Add a full level of multiple nodes
    pub(crate) fn add_all(&mut self, node_type: NodeType) {
        let count = self.depth * CHILDREN_PER_NODE;

        match node_type {
            NodeType::Root => todo!(),
            NodeType::Internal => todo!(),
            NodeType::Leaf => todo!(),
            NodeType::Null => self.nodes.resize(count, Node::new_null(self.depth)),
        }
    }
}

/*
depth 0:
    0: 1->0, 1->1

depth 1:
    0: 2->0, 2->1
    1: 2->2, 2->3

depth 2:
    0: 2->0, 2->1
    1: 2->2, 2->3
    2: 2->4, 2->5
    3: 2->6, 2->7
*/
#[derive(Debug)]
pub struct Tree<const CHILDREN_PER_NODE: usize> {
    levels: [TreeLevel<2>; Tree::MAX_DEPTH + 1],
    _marker: std::marker::PhantomData<[(); CHILDREN_PER_NODE]>
}

pub type BinaryTree = Tree<2>;

impl Default for BinaryTree {
    fn default() -> Self {
        BinaryTree {
            levels: Default::default(),
            ..Default::default()
        }
    }
}

impl<const CHILDREN_PER_NODE: usize> BinaryTree {
    const MAX_DEPTH: usize = 2;

    pub fn with_depth(desired_depth: usize) -> Option<BinaryTree> {
        if desired_depth > Self::MAX_DEPTH {
            return None;
        }

        let mut tree = BinaryTree::default();

        // Allocate the root
        tree.levels[0].add(NodeType::Root);

        // All other levels
        for level_idx in 1..=desired_depth {
            let depth = level_idx;
            tree.levels[level_idx].depth = depth;
            tree.levels[level_idx].add_all(NodeType::Null);
        }

        Some(tree)
    }

    pub(crate) fn get_root(&self) -> Option<&Node> {
        self.levels[0].nodes.get(0)
    }

    pub(crate) fn get_root_mut(&mut self) -> Option<&mut Node> {
        self.levels[0].nodes.get_mut(0)
    }

    pub(crate) fn get_parent_of(&self, node: &Node) -> Option<Box<Node>> {
        todo!()
    }

    pub(crate) fn is_leaf(&self, node: &Node) -> bool {
        todo!()
    }

    pub(crate) fn already_split(&self, node: &Node) -> bool {
        todo!()
    }
}
