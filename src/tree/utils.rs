use super::node::Node;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Color {
    Red,
    Black,
}

pub fn get_color(node_or_leaf: Option<&Box<Node>>) -> Color {
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

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
