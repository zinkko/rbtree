use std::fmt;

mod iter;
mod node;
mod utils;

use node::Node;
use utils::{Color, get_color, Direction, RotationType};



fn recursive_insert(node: &mut Node, value: i32) -> InsertReturn {
    let direction = if value < node.value { Direction::Left } else { Direction::Right };
    let uncle_color = get_color(node.get_child_as_ref(direction.opposite()));
    let mut next = node.get_child(direction);
    if next.is_none() {
        node.set_child(direction, Node::new(Color::Red, value));
        
        return match node.color {
            Color::Black => InsertReturn::Done,
            Color::Red => InsertReturn::Parent(direction),
        };
    }

    let state = recursive_insert(next.as_mut().unwrap(), value);
    match state {
        InsertReturn::Done => InsertReturn::Done,
        InsertReturn::Node => {
            if node.color == Color::Black {
                InsertReturn::Done
            } else {
                InsertReturn::Parent(direction)
            }
        },
        InsertReturn::Parent(child_direction) => {
            if uncle_color == Color::Red {
                next.unwrap().color = Color::Black;
                node.get_child(direction.opposite()).unwrap().color = Color::Black;
                node.color = Color::Red;
                InsertReturn::Node
            } else {
                // case 4 & 5, inner grandchild
                if child_direction != direction {
                    InsertReturn::Rotate(RotationType::Double(direction.opposite()))
                // case 5
                } else {
                    InsertReturn::Rotate(RotationType::Single(direction.opposite()))
                }
            }
        },
        InsertReturn::Rotate(rotation_type) => {
            let rotation_dir = rotation_type.get_direction();
            let child = *(node.remove_child(direction).take().unwrap());
            let mut rotated_node = child.rotate(rotation_type);
            rotated_node.color = Color::Black;
            rotated_node.get_child(rotation_dir).expect("The parent should have been rotated here").color = Color::Red;
            
            node.set_child(direction, rotated_node);
            InsertReturn::Done
        },
    }
}

enum InsertReturn {
    Done,
    Node,
    Parent(Direction),
    Rotate(RotationType),
}

pub struct Tree {
    root: Option<Box<Node>>,
}

impl Tree {
    pub fn new() -> Tree {
        Tree { root: None }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn contains(&self, value: i32) -> bool {
        let mut next = self.root.as_ref();
        while let Some(node) = next {
            if node.value == value {
                return true;
            } else if value < node.value {
                next = node.left.as_ref();
            } else {
                next = node.right.as_ref();
            }
        }
        false
    }

    pub fn insert(&mut self, value: i32) {
        if self.root.is_none() {
            self.root = Some(Box::new(Node {
                color: Color::Black,
                value: value,
                left: None,
                right: None,
            }));
            return;
        }
        let insert_result = recursive_insert(self.root.as_mut().unwrap(), value);
        match insert_result {
            InsertReturn::Done => {},
            InsertReturn::Node => {},
            InsertReturn::Parent(_) => {
                self.root.as_mut().unwrap().color = Color::Black;
            },
            InsertReturn::Rotate(rotation_type) => {
                let rotation_dir = rotation_type.get_direction();
                let old_root = *(self.root.take().unwrap());
                let mut new_root = old_root.rotate(rotation_type);
                new_root.color = Color::Black;
                new_root.get_child(rotation_dir).expect("The parent should have been rotated here").color = Color::Red;
                self.root = Some(Box::new(new_root));
            }
        }
    }
}

impl IntoIterator for Tree {
    type Item = i32;
    type IntoIter = iter::IntoIter;

    fn into_iter(self) -> iter::IntoIter {
        iter::IntoIter::new(self)
    }
}

// helper function for fmt::Debug
fn fmt_subtree(node: &Box<Node>, formatter: &mut fmt::Formatter, indent: usize) -> fmt::Result {
    let indent_size = 2;
    formatter.write_fmt(format_args!("{:width$} {:?} {:?}\n", "", node.color, node.value, width=indent))?;

    if node.left.is_none() && node.right.is_none() {
        return Ok(());
    }

    match &node.left {
        Some(left_node) => fmt_subtree(left_node, formatter, indent + indent_size)?,
        None => formatter.write_fmt(format_args!("{:width$} Leaf\n", "", width=indent+indent_size))?,
    };
    match &node.right {
        Some(right_node) => fmt_subtree(right_node, formatter, indent + indent_size),
        None => formatter.write_fmt(format_args!("{:width$} Leaf\n", "", width=indent+indent_size)),
    }
}

impl fmt::Debug for Tree {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match &self.root {
            Some(root_node) => fmt_subtree(root_node, formatter, 0),
            None => formatter.write_str("Empty tree\n"),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    mod tools {
        use super::super::*;

        pub fn assert_no_red_violations(tree: &Tree) {
            match &tree.root {
                Some(node) => check_red_violations(&node),
                None => {},
            }
        }

        fn check_red_violations(node: &Box<Node>) {
            if node.color == Color::Red {
                assert_eq!(get_color(node.left.as_ref()), Color::Black, "Child of red node must be black");
                assert_eq!(get_color(node.right.as_ref()), Color::Black, "Child of red node must be black");
            }

            if let Some(left_node) = node.left.as_ref() {
                check_red_violations(left_node);
            }
            if let Some(right_node) = node.right.as_ref() {
                check_red_violations(&right_node);
            }
        }

        pub fn assert_no_black_violations(tree: &Tree) {
            check_black_violations(tree.root.as_ref());
        }

        fn check_black_violations(node_or_leaf: Option<&Box<Node>>) -> i32 {
            if let Some(node) = node_or_leaf {
                let black_height_left = check_black_violations(node.left.as_ref());
                let black_height_right = check_black_violations(node.right.as_ref());
                
                assert_eq!(black_height_left, black_height_right, "Paths to leaves must contain same amount of black nodes. Violations in subtree of {:?} node with value {:?}", node.color, node.value);
                
                match node.color {
                    Color::Red => black_height_left,
                    Color::Black => black_height_left + 1,
                }
            } else {
                0
            }
        }

        pub fn assert_tree_size(tree: &Tree, expected_size: usize) {
            assert_eq!(subtree_size(tree.root.as_ref()), expected_size, "Tree was not the right size");
        }

        fn subtree_size(node_or_leaf: Option<&Box<Node>>) -> usize {
            match node_or_leaf {
                Some(node) => subtree_size(node.left.as_ref()) + subtree_size(node.right.as_ref()) + 1,
                None => 0,
            }
        }
    }

    #[test]
    fn test_new_tree_is_empty() {
        assert!(Tree::new().is_empty());
    }

    #[test]
    fn test_after_insert_tree_not_empty() {
        let mut tree = Tree::new();
        tree.insert(8);
        assert!(!tree.is_empty());
    }

    #[test]
    fn test_contains() {
        let t = Tree {
            root: Some(Box::new(Node {
                color: Color::Red,
                value: 5,
                left: Some(Box::new(Node {
                    color: Color::Red,
                    value: 3,
                    left: Some(Box::new(Node { color: Color::Red, value: 1, left: None, right: None })),
                    right: Some(Box::new(Node { color: Color::Red, value: 4, left: None, right: None })),
                })),
                right: Some(Box::new(Node {
                    color: Color::Red,
                    value: 8,
                    left: Some(Box::new(Node { color: Color::Red, value: 6, left: None, right: None})),
                    right: None,
                })),
            })),
        };
        assert!(t.contains(5));
        assert!(t.contains(6));
        assert!(t.contains(1));
        assert!(t.contains(4));

        assert!(!t.contains(2));
        assert!(!t.contains(7));
    }

    #[test]
    fn test_insert_1() {
        let mut t = Tree::new();
        t.insert(3);
        t.insert(6);
        t.insert(1);

        tools::assert_no_red_violations(&t);
        tools::assert_no_black_violations(&t);

        tools::assert_tree_size(&t, 3);
    }

    #[test]
    fn test_insert_2() {
        let mut tree = Tree::new();
        let values = vec![45, 13, 54, 14, 77, 12, 0, -3, 43, 111, 124, 55, 3, 1, 211434, 3];
        let expected_size = values.len();
        for i in values {
            tree.insert(i);
        }

        tools::assert_no_red_violations(&tree);
        tools::assert_no_black_violations(&tree);
        tools::assert_tree_size(&tree, expected_size);
    }

    #[test]
    fn test_into_iter() {
        let mut tree = Tree::new();
        let values = vec![145, -1243, 54, -123, 434, 13];
        for i in values {
            tree.insert(i);
        }

        assert_eq!(tree.into_iter().collect::<Vec<i32>>(), vec![-1243, -123, 13, 54, 145, 434]);
    }
}
