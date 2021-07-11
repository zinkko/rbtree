
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    value: i32,
}

pub struct Tree {
    root: Option<Node>,
}

impl Tree {
    pub fn new() -> Tree {
        Tree { root: None }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(Tree::new().is_empty());
    }
}
