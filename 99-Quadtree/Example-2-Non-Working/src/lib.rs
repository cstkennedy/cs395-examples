use std::rc::Rc;

#[derive(Debug)]
pub struct BinaryTree {
    root: Rc<Node>,
}

impl BinaryTree {
    const MAX_DEPTH: u8 = 2;

    pub fn until_depth(desired_depth: u8) -> BinaryTree {
        let mut root = Node::new();

        let root = root.split_recursive();

        BinaryTree {
            root
        }
    }
}

#[derive(Debug)]
pub struct Node {
    depth: u8,

    parent: Option<Rc<RefCell<Node>>>,

    left: Option<Rc<Node>>,
    right: Option<Rc<Node>>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            depth: 0,
            parent: None,
            left: None,
            right: None,
        }
    }

    fn new_at_depth(depth: u8, parent: Rc<Node>) -> Self {
        Self {
            depth,
            parent: Some(parent),
            left: None,
            right: None,
        }
    }

    pub fn get_parent(&self) -> Option<Rc<Node>> {
        todo!()
    }

    pub fn is_leaf(&self) -> bool {
        todo!()
    }

    pub fn already_split(&self) -> bool {
        self.left.is_some() || self.right.is_some() 
    }

    pub fn split_recursive(self) -> Rc<Self> {
        if self.already_split() {
            return Rc::new(self);
        }

        if self.depth == BinaryTree::MAX_DEPTH {
            return Rc::new(self);
        }

        let current_depth = self.depth;
        eprintln!("{current_depth}");

        /*
        let mut left = Rc::new(Node::new_at_depth(current_depth + 1));
        let mut right = Rc::new(Node::new_at_depth(current_depth + 1));

        Rc::get_mut(&mut left).unwrap().split_recursive();
        Rc::get_mut(&mut right).unwrap().split_recursive();
        */

        let mut rc_self = Rc::new(self);
        let left = Node::new_at_depth(current_depth + 1, rc_self.clone());
        let right = Node::new_at_depth(current_depth + 1, rc_self.clone());

        let left = left.split_recursive();
        let right = right.split_recursive();

        println!("{:#?}", rc_self);

        Rc::get_mut(&mut rc_self).unwrap().left = Some(left);
        Rc::get_mut(&mut rc_self).unwrap().right = Some(right);

        rc_self
    }
}

