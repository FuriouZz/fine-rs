use fine_render::prelude::Geometry;
use fine_transform::{Node, NodeVisitor};
pub struct Mesh {
    pub node: Node,
    pub geometry: Geometry,
}

impl NodeVisitor for Mesh {
    #[inline]
    fn update_world_matrix(&mut self, parent: Option<&Node>) {
        self.node.update_world_matrix(parent);
    }
}