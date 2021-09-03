use fine_render::prelude::{wgpu, OPENGL_TO_WGPU_MATRIX};
use fine_transform::{Node, NodeVisitor};
use glam::Mat4;

pub struct OrthographicOptions {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
    pub zoom: f32,
}

impl Default for OrthographicOptions {
    fn default() -> Self {
        Self {
            left: -1.0,
            right: 1.0,
            bottom: -1.0,
            top: 1.0,
            zoom: 1.0,
            near: 0.0,
            far: 1000.0,
        }
    }
}

pub struct PerspectiveOptions {
    pub fovy: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl Default for PerspectiveOptions {
    fn default() -> Self {
        Self {
            fovy: 45.0,
            aspect: 1.0,
            near: 0.01,
            far: 100.0,
        }
    }
}

pub struct Camera {
    pub view: Mat4,
    pub projection: Mat4,
    pub projection_view: Mat4,
    pub node: Node,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            view: Mat4::IDENTITY,
            projection: Mat4::IDENTITY,
            projection_view: Mat4::IDENTITY,
            node: Node::new(),
        }
    }
}

impl Camera {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn perspective(&mut self, options: PerspectiveOptions) {
        self.projection = Mat4::perspective_rh(options.fovy, options.aspect, options.near, options.far);
        self.node.invalidate();
    }

    pub fn orthographic(&mut self, options: OrthographicOptions) {
        self.projection = Mat4::orthographic_rh(
                options.left / options.zoom,
                options.right / options.zoom,
                options.bottom / options.zoom,
                options.top / options.zoom,
                options.near,
                options.far,
            );
        self.node.invalidate();
    }
}

impl NodeVisitor for Camera {
    #[inline]
    fn update_world_matrix(&mut self, parent: Option<&Node>) {
        let is_invalid = self.node.is_invalid(parent);
        if is_invalid {
            self.node.update_world_matrix(parent);
            self.view = self.node.get_world_matrix().inverse();

            self.projection_view = self.projection * self.view;
        }
    }
}
