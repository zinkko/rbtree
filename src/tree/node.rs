
use super::*;

pub struct Node<T: Ord> {
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
    pub value: T,

    pub color: Color,
}

impl<T: Ord> Node<T> {
    pub fn new(color: Color, value: T) -> Node<T> {
        Node { color: color, value: value, left: None, right: None }
    }

    pub fn get_child(&mut self, dir: Direction) -> Option<&mut Box<Node<T>>> {
        match dir {
            Direction::Left => self.left.as_mut(),
            Direction::Right => self.right.as_mut(),
        }
    }

    pub fn get_child_as_ref(&mut self, dir: Direction) -> Option<&Box<Node<T>>> {
        match dir {
            Direction::Left => self.left.as_ref(),
            Direction::Right => self.right.as_ref(),
        }
    }

    pub fn set_child(&mut self, dir: Direction, node: Node<T>) {
        match dir {
            Direction::Left => self.left = Some(Box::new(node)),
            Direction::Right => self.right = Some(Box::new(node)),
        }
    }

    pub fn set_child_or_leaf(&mut self, dir: Direction, child: Option<Box<Node<T>>>) {
        match dir {
            Direction::Left => self.left = child,
            Direction::Right => self.right = child,
        }
    }

    pub fn remove_child(&mut self, dir: Direction) -> Option<Box<Node<T>>> {
        match dir {
            Direction::Left => {
                self.left.take()
            },
            Direction::Right => {
                self.right.take()
            }
        }
    }

    pub fn is_black(&self) -> bool {
        self.color == Color::Black
    }

    pub fn rotate(self, rtype: RotationType) -> Node<T> {
        match rtype {
            RotationType::Single(dir) => self.rotate_once(dir),
            RotationType::Double(dir) => self.rotate_twice(dir),
        }
    }

    fn rotate_once(mut self, dir: Direction) -> Node<T> {
        let u = self.remove_child(dir);
        let mut p = *(self.remove_child(dir.opposite()).expect("Rotation need one child"));
        let n = p.remove_child(dir.opposite());
        let s = p.remove_child(dir);

        self.set_child_or_leaf(dir, u);
        self.set_child_or_leaf(dir.opposite(), s);
        
        p.set_child(dir, self);
        p.set_child_or_leaf(dir.opposite(), n);

        p
    }

    fn rotate_twice(mut self, dir: Direction) -> Node<T> {
        let u = self.remove_child(dir);
        let mut p = *(self.remove_child(dir.opposite()).expect("Double rotation needs the parent"));
        let mut n = *(p.remove_child(dir).expect("Double rotation needs inner grandchild"));
        let s = p.remove_child(dir.opposite());

        let b1 = n.remove_child(dir.opposite());
        let b2 = n.remove_child(dir);

        p.set_child_or_leaf(dir.opposite(), s);
        p.set_child_or_leaf(dir, b1);
        
        self.set_child_or_leaf(dir.opposite(), b2);
        self.set_child_or_leaf(dir, u);

        n.set_child(dir.opposite(), p);
        n.set_child(dir, self);

        n
    }
}