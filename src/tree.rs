
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    value: i32,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tree_is_empty() {
        assert!(Tree::new().is_empty());
    }

    #[test]
    fn test_contains() {
        let t = Tree {
            root: Some(Box::new(Node {
                value: 5,
                left: Some(Box::new(Node {
                    value: 3,
                    left: Some(Box::new(Node { value: 1, left: None, right: None })),
                    right: Some(Box::new(Node { value: 4, left: None, right: None })),
                })),
                right: Some(Box::new(Node {
                    value: 8,
                    left: Some(Box::new(Node { value: 6, left: None, right: None})),
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
}
