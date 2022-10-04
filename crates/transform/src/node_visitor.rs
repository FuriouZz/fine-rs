use crate::Node;

pub trait NodeVisitor {
    fn update_world_matrix(&mut self, parent: Option<&Node>);
}