
use super::*;

enum IterTask<T: Ord + Copy> {
    Value(T),
    Node(Box<Node<T>>),
}

pub struct IntoIter<T: Ord + Copy> {
    tasks: Vec<IterTask<T>>,
}

fn add_tasks<T: Ord + Copy>(tasks: &mut Vec<IterTask<T>>, node: Box<Node<T>>) {
    if let Some(right_node) = node.right {
        tasks.push(IterTask::Node(right_node));
    }
    tasks.push(IterTask::Value(node.value));
    if let Some(left_node) = node.left {
        tasks.push(IterTask::Node(left_node));
    }
}

impl<T: Ord + Copy> IntoIter<T> {
    pub fn new(tree: RBTree<T>) -> IntoIter<T> {
        let mut tasks = Vec::new();
        if let Some(root_node) = tree.root {
            add_tasks::<T>(&mut tasks, root_node);
        }
        IntoIter { tasks: tasks }
    }
}

impl<T: Ord + Copy> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(task) = self.tasks.pop() {
            match task {
                IterTask::Value(v) => return Some(v),
                IterTask::Node(node) => {
                    add_tasks(&mut self.tasks, node);
                }
            }
        }
        None
    }
}
