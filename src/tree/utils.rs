use super::node::Node;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Color {
    Red,
    Black,
}

pub fn get_color<T: Ord>(node_or_leaf: Option<&Box<Node<T>>>) -> Color {
    match node_or_leaf {
        Some(node) => node.color,
        None => Color::Black,
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub enum RotationType {
    Single(Direction),
    Double(Direction),
}

impl RotationType {
    pub fn get_direction(&self) -> Direction {
        match self {
            Self::Single(d) => *d,
            Self::Double(d) => *d,
        }
    }
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
