#[derive(Debug)]
pub struct BinaryTree {
    root: Node,
}

impl BinaryTree {
    const MAX_DEPTH: u8 = 2;

    pub fn until_depth(desired_depth: u8) -> BinaryTree {
        let mut root = Node::new();

        root.split_recursive();

        BinaryTree {
            root
        }
    }
}

#[derive(Debug)]
pub struct Node {
    depth: u8,

    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            depth: 0,
            left: None,
            right: None,
        }
    }

    fn new_at_depth(depth: u8) -> Self {
        Self {
            depth,
            left: None,
            right: None,
        }
    }

    pub fn get_parent(&self) -> Option<Box<Node>> {
        todo!()
    }

    pub fn is_leaf(&self) -> bool {
        todo!()
    }

    pub fn already_split(&self) -> bool {
        self.left.is_some() || self.right.is_some() 
    }

    pub fn split_recursive(&mut self) {
        if self.already_split() {
            return;
        }

        if self.depth == BinaryTree::MAX_DEPTH {
            return;
        }

        let current_depth = self.depth;
        let mut left = Box::new(Node::new_at_depth(current_depth + 1));
        let mut right = Box::new(Node::new_at_depth(current_depth + 1));

        left.split_recursive();
        right.split_recursive();

        self.left = Some(left);
        self.right = Some(right);
    }
}

