use std::iter::Peekable;
use std::ops::Add;
use std::str::Chars;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NodeId(usize);

#[derive(Clone, Debug, Default)]
pub struct BinaryTree {
    nodes: Vec<Node>,
    root_node_id: Option<NodeId>,
}

#[derive(Clone, Debug)]
pub enum Node {
    Leaf {
        node_id: NodeId,
        parent: Option<NodeId>,
        value: usize,
    },
    Node {
        node_id: NodeId,
        parent: Option<NodeId>,
        left_node_id: NodeId,
        right_node_id: NodeId,
    },
}

impl Add for NodeId {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl FromStr for BinaryTree {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn read_expected(
            characters: &mut Peekable<Chars<'_>>,
            expected_character: char,
        ) -> Result<(), anyhow::Error> {
            match characters.next() {
                Some(value) if value == expected_character => Ok(()),
                Some(value) => Err(anyhow::anyhow!(
                    "Unexpected input. Expected '{expected_character}', received '{value}'",
                    expected_character = expected_character,
                    value = value
                )),
                None => Err(anyhow::anyhow!(
                    "Unexpected input. Expected '{expected_character}', received no character",
                    expected_character = expected_character
                )),
            }
        }

        fn recurse(
            binary_tree: &mut BinaryTree,
            characters: &mut Peekable<Chars<'_>>,
        ) -> Result<NodeId, anyhow::Error> {
            if let Some('0'..='9') = characters.peek() {
                let mut value = 0;
                while let Some(character) = characters.next_if(char::is_ascii_digit) {
                    value = value * 10 + (character as usize - '0' as usize);
                }
                Ok(binary_tree.new_leaf(value))
            } else {
                read_expected(characters, '[')?;
                let left_node_id = recurse(binary_tree, characters)?;
                read_expected(characters, ',')?;
                let right_node_id = recurse(binary_tree, characters)?;
                read_expected(characters, ']')?;
                let node = binary_tree.new_node(left_node_id, right_node_id)?;
                binary_tree.set_root(node);
                Ok(node)
            }
        }

        let mut binary_tree = BinaryTree::default();
        recurse(&mut binary_tree, &mut s.chars().peekable())?;

        Ok(binary_tree)
    }
}

impl BinaryTree {
    fn next_node_id(&self) -> NodeId {
        NodeId(self.nodes.len())
    }

    pub fn new_leaf(&mut self, value: usize) -> NodeId {
        let node_id = self.next_node_id();
        let leaf = Node::Leaf {
            node_id,
            parent: None,
            value,
        };
        self.nodes.push(leaf);
        node_id
    }

    pub fn new_node(
        &mut self,
        left_node_id: NodeId,
        right_node_id: NodeId,
    ) -> Result<NodeId, anyhow::Error> {
        anyhow::ensure!(self.nodes.len() > left_node_id.0);
        anyhow::ensure!(self.get_parent(left_node_id).is_none());
        anyhow::ensure!(self.nodes.len() > right_node_id.0);
        anyhow::ensure!(self.get_parent(right_node_id).is_none());

        let node_id = self.next_node_id();
        self.set_parent(left_node_id, node_id)?;
        self.set_parent(right_node_id, node_id)?;
        let leaf = Node::Node {
            node_id,
            parent: None,
            left_node_id,
            right_node_id,
        };
        self.nodes.push(leaf);
        Ok(node_id)
    }

    pub fn get_node(&self, node_id: NodeId) -> Result<&Node, anyhow::Error> {
        if let Some(node) = self.nodes.get(node_id.0) {
            Ok(node)
        } else {
            Err(anyhow::anyhow!(
                "{:?} should be registered in the binary tree",
                node_id
            ))
        }
    }

    pub fn set_root(&mut self, root_node_id: NodeId) -> Option<NodeId> {
        self.root_node_id.replace(root_node_id)
    }

    pub fn merge(&mut self, other: &Self) -> Result<(), anyhow::Error> {
        let self_root_node_id = if let Some(root_node_id) = self.root_node_id {
            root_node_id
        } else {
            return Err(anyhow::anyhow!("The tree does not have a root defined"));
        };
        let other_root_node_id = if let Some(root_node_id) = other.root_node_id {
            root_node_id
        } else {
            return Err(anyhow::anyhow!("The tree does not have a root defined"));
        };

        let offset_node_id = self.next_node_id();

        for other_node in &other.nodes {
            match other_node {
                Node::Leaf {
                    mut node_id,
                    mut parent,
                    value,
                } => {
                    node_id = node_id + offset_node_id;
                    parent = parent.map(|parent_node_id| parent_node_id + offset_node_id);
                    self.nodes.push(Node::Leaf {
                        node_id,
                        parent,
                        value: *value,
                    });
                }
                Node::Node {
                    mut node_id,
                    mut parent,
                    mut left_node_id,
                    mut right_node_id,
                } => {
                    node_id = node_id + offset_node_id;
                    parent = parent.map(|parent_node_id| parent_node_id + offset_node_id);
                    left_node_id = left_node_id + offset_node_id;
                    right_node_id = right_node_id + offset_node_id;
                    self.nodes.push(Node::Node {
                        node_id,
                        parent,
                        left_node_id,
                        right_node_id,
                    });
                }
            }
        }

        let new_root_node_id =
            self.new_node(self_root_node_id, other_root_node_id + offset_node_id)?;
        self.set_root(new_root_node_id);
        Ok(())
    }

    pub fn magnitude(&self) -> Result<usize, anyhow::Error> {
        if let Some(root_node_id) = self.root_node_id {
            self.get_node(root_node_id)?.magnitude(self)
        } else {
            Err(anyhow::anyhow!("The tree does not have a root defined"))
        }
    }

    pub fn left_leaf(&self, mut node_id: NodeId) -> Option<&Node> {
        let mut maybe_node_to_descend_right: Option<&Node> = None;
        while let Some(parent) = self
            .get_parent(node_id)
            .and_then(|parent_node_id| self.get_node(parent_node_id).ok())
        {
            match parent.right_child(self) {
                Some(right_child) => {
                    if right_child.node_id() == node_id {
                        if let Some(left_child) = parent.left_child(self) {
                            maybe_node_to_descend_right.replace(left_child);
                        }
                        break;
                    }
                    node_id = parent.node_id();
                }
                None => {
                    node_id = parent.node_id();
                }
            }
        }
        if let Some(mut node_to_descend_right) = maybe_node_to_descend_right {
            loop {
                if let Some(right_child) = node_to_descend_right.right_child(self) {
                    node_to_descend_right = right_child;
                } else {
                    break;
                }
            }
            maybe_node_to_descend_right.replace(node_to_descend_right);
        }
        maybe_node_to_descend_right
    }

    pub fn right_leaf(&self, mut node_id: NodeId) -> Option<&Node> {
        let mut maybe_node_to_descend_left: Option<&Node> = None;
        while let Some(parent) = self
            .get_parent(node_id)
            .and_then(|parent_node_id| self.get_node(parent_node_id).ok())
        {
            match parent.left_child(self) {
                Some(left_child) => {
                    if left_child.node_id() == node_id {
                        if let Some(right_child) = parent.right_child(self) {
                            maybe_node_to_descend_left.replace(right_child);
                        }
                        break;
                    }
                    node_id = parent.node_id();
                }
                None => {
                    node_id = parent.node_id();
                }
            }
        }
        if let Some(mut node_to_descend_left) = maybe_node_to_descend_left {
            while let Some(left_child) = node_to_descend_left.left_child(self) {
                node_to_descend_left = left_child;
            }
            maybe_node_to_descend_left.replace(node_to_descend_left);
        }
        maybe_node_to_descend_left
    }

    fn get_node_mut(&mut self, node_id: NodeId) -> Result<&mut Node, anyhow::Error> {
        if let Some(node) = self.nodes.get_mut(node_id.0) {
            Ok(node)
        } else {
            Err(anyhow::anyhow!(
                "{:?} should be registered in the binary tree",
                node_id
            ))
        }
    }

    fn get_parent(&self, node_id: NodeId) -> Option<NodeId> {
        match self.get_node(node_id).ok()? {
            Node::Leaf { parent, .. } | Node::Node { parent, .. } => *parent,
        }
    }

    fn set_parent(
        &mut self,
        node_id: NodeId,
        parent_node_id: NodeId,
    ) -> Result<Option<NodeId>, anyhow::Error> {
        match self.get_node_mut(node_id)? {
            Node::Leaf { parent, .. } | Node::Node { parent, .. } => {
                Ok(parent.replace(parent_node_id))
            }
        }
    }

    pub fn visit(&self) -> Result<String, anyhow::Error> {
        if let Some(root_node_id) = self.root_node_id {
            self.visit_rec(root_node_id)
        } else {
            Err(anyhow::anyhow!("The tree does not have a root defined"))
        }
    }

    fn visit_rec(&self, node_id: NodeId) -> Result<String, anyhow::Error> {
        self.get_node(node_id).and_then(|node| match node {
            Node::Leaf { value, .. } => Ok(format!("{}", value)),
            Node::Node {
                left_node_id,
                right_node_id,
                ..
            } => Ok(format!(
                "[{}, {}]",
                self.visit_rec(*left_node_id)?,
                self.visit_rec(*right_node_id)?,
            )),
        })
    }
}

impl Node {
    fn node_id(&self) -> NodeId {
        match self {
            Self::Leaf { node_id, .. } | Self::Node { node_id, .. } => *node_id,
        }
    }

    fn magnitude(&self, binary_tree: &BinaryTree) -> Result<usize, anyhow::Error> {
        match self {
            Self::Leaf { value, .. } => Ok(*value),
            Self::Node {
                left_node_id,
                right_node_id,
                ..
            } => {
                let left_value = binary_tree
                    .get_node(*left_node_id)?
                    .magnitude(binary_tree)?;
                let right_value = binary_tree
                    .get_node(*right_node_id)?
                    .magnitude(binary_tree)?;
                Ok(3 * left_value + 2 * right_value)
            }
        }
    }

    fn get_parent<'b>(&self, binary_tree: &'b BinaryTree) -> Option<&'b Self> {
        match self {
            Self::Leaf { parent, .. } | Self::Node { parent, .. } => {
                parent.and_then(|parent_node_id| binary_tree.get_node(parent_node_id).ok())
            }
        }
    }

    fn get_parent_mut<'b>(&self, binary_tree: &'b mut BinaryTree) -> Option<&'b mut Self> {
        match self {
            Self::Leaf { parent, .. } | Self::Node { parent, .. } => {
                parent.and_then(move |parent_node_id| binary_tree.get_node_mut(parent_node_id).ok())
            }
        }
    }

    pub fn left_child<'b>(&self, binary_tree: &'b BinaryTree) -> Option<&'b Self> {
        if let Self::Node { left_node_id, .. } = self {
            binary_tree.get_node(*left_node_id).ok()
        } else {
            None
        }
    }

    pub fn right_child<'b>(&self, binary_tree: &'b BinaryTree) -> Option<&'b Self> {
        if let Self::Node { right_node_id, .. } = self {
            binary_tree.get_node(*right_node_id).ok()
        } else {
            None
        }
    }
}
