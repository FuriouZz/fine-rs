use fine_transform::{Node, NodeVisitor};
use glam::Mat4;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

pub enum CameraProjection {
    Perspective(PerspectiveOptions),
    Orthographic(OrthographicOptions),
}

impl Default for CameraProjection {
    fn default() -> Self {
        Self::Orthographic(Default::default())
    }
}

pub struct Camera {
    pub node: Node,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    projection_view_matrix: Mat4,
    projection_invalid: bool,
    projection: CameraProjection,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            view_matrix: Mat4::IDENTITY,
            projection_matrix: Mat4::IDENTITY,
            projection_view_matrix: Mat4::IDENTITY,
            node: Node::new(),
            projection_invalid: false,
            projection: Default::default(),
        }
    }
}

impl Camera {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_projection(&self) -> &CameraProjection {
        &self.projection
    }

    pub fn get_mut_projection(&mut self) -> &mut CameraProjection {
        let projection = &mut self.projection;
        self.projection_invalid = true;
        self.node.invalidate();
        projection
    }

    pub fn get_view_matrix(&self) -> &Mat4 {
        &self.view_matrix
    }

    pub fn get_projection_matrix(&self) -> &Mat4 {
        &self.projection_matrix
    }

    pub fn get_projection_view_matrix(&self) -> &Mat4 {
        &self.projection_view_matrix
    }

    pub fn invalidate(&mut self) {
        self.projection_invalid = true;
        self.node.invalidate();
    }

    pub fn update_projection(&mut self) {
        if self.projection_invalid {
            match &self.projection {
                CameraProjection::Orthographic(options) => {
                    self.projection_matrix = Mat4::orthographic_rh(
                        options.left / options.zoom,
                        options.right / options.zoom,
                        options.bottom / options.zoom,
                        options.top / options.zoom,
                        options.near,
                        options.far,
                    );
                }
                CameraProjection::Perspective(options) => {
                    self.projection_matrix = Mat4::perspective_rh(options.fovy, options.aspect, options.near, options.far);
                }
            }
        }
    }
}

impl NodeVisitor for Camera {
    #[inline]
    fn update_world_matrix(&mut self, parent: Option<&Node>) {
        if self.node.is_invalid() {
            self.node.update_world_matrix(parent);
            self.view_matrix = self.node.get_world_matrix().inverse();
            self.projection_view_matrix = self.projection_matrix * self.view_matrix;
        }
    }
}
