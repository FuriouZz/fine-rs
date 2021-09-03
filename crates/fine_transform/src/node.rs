use glam::Mat4;

use crate::transform::Transform;

#[derive(Debug)]
pub struct Node {
    name: String,
    transform: Transform,
    local_matrix: Mat4,
    world_matrix: Mat4,
    invalid_local_matrix: bool,
    invalid_world_matrix: bool,
    world_id: u16,
    world_parent_id: u16,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            name: "Node".into(),
            transform: Transform::new(),
            local_matrix: Mat4::IDENTITY,
            invalid_local_matrix: false,
            world_matrix: Mat4::IDENTITY,
            invalid_world_matrix: false,
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
        self.invalidate();
    }

    pub fn get_world_matrix(&self) -> &Mat4 {
        &self.world_matrix
    }

    pub fn invalidate(&mut self) {
        self.invalid_local_matrix = true;
        self.invalid_world_matrix = true;
    }

    pub fn is_invalid(&self, parent: Option<&Node>) -> bool {
        self.invalid_local_matrix
            || parent.map_or(self.invalid_world_matrix, |p| {
                self.world_parent_id != p.world_id
            })
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

    pub fn update_world_matrix(&mut self, parent: Option<&Node>) {
        self.update_local_matrix();

        if self.is_invalid(parent) {
            if let Some(parent) = parent {
                self.world_matrix = parent.world_matrix.mul_mat4(&self.local_matrix);
                self.world_parent_id = parent.world_id;
            } else {
                self.world_matrix = self.local_matrix.clone();
            }
            self.invalid_world_matrix = false;
            self.world_id += 1;
        }
    }
}
