use glam::Mat4;

use crate::transform::Transform;

#[derive(Debug)]
pub struct Node {
    transform: Transform,
    local_matrix: Mat4,
    invalid_local_matrix: bool,
    world_matrix: Mat4,
    world_id: u16,
    world_parent_id: u16,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            transform: Transform::new(),
            local_matrix: Mat4::IDENTITY,
            invalid_local_matrix: false,
            world_matrix: Mat4::IDENTITY,
            world_id: 0,
            world_parent_id: 0,
        }
    }
}

impl Node {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_mat4(local_matrix: Mat4) -> Self {
        let (scale, rotation, translation) = local_matrix.to_scale_rotation_translation();
        Self {
            local_matrix,
            transform: Transform {
                scale,
                rotation,
                translation,
            },
            ..Default::default()
        }
    }

    pub fn get_transform(&self) -> &Transform {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
        self.invalid_local_matrix = true;
    }

    pub fn get_world_matrix(&self) -> &Mat4 {
        &self.world_matrix
    }

    pub fn is_invalid(&self) -> bool {
        self.invalid_local_matrix
    }

    pub fn invalidate(&mut self) {
        self.invalid_local_matrix = true;
    }

    pub fn update_local_matrix(&mut self) {
        if self.invalid_local_matrix {
            self.local_matrix = Mat4::from_scale_rotation_translation(
                self.transform.scale,
                self.transform.rotation,
                self.transform.translation,
            );
        }
    }

    pub fn update_world_matrix(&mut self, parent: &Node) {
        self.update_local_matrix();

        if self.world_parent_id != parent.world_id {
            self.world_matrix = parent.world_matrix.mul_mat4(&self.local_matrix);
            self.world_parent_id = parent.world_id;
            self.world_id += 1;
        }
    }
}
