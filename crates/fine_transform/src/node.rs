use fine_math::Mat4;

use crate::transform::Transform;

#[derive(Debug)]
pub struct Node {
    transform: Transform,
    local_matrix: Mat4,
    invalid_local_matrix: bool,
    world_matrix: Mat4,
    invalid_world_matrix: bool,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            transform: Transform::new(),
            local_matrix: Mat4::IDENTITY,
            invalid_local_matrix: false,
            world_matrix: Mat4::IDENTITY,
            invalid_world_matrix: false,
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

    pub fn get_mut_transform(&mut self) -> &mut Transform {
        self.invalidate();
        &mut self.transform
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
        self.invalidate();
    }

    pub fn get_world_matrix(&self) -> &Mat4 {
        &self.world_matrix
    }

    pub fn is_invalid(&self) -> bool {
        self.invalid_local_matrix
    }

    pub fn invalidate(&mut self) {
        self.invalid_local_matrix = true;
        self.invalid_world_matrix = true;
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

        if self.invalid_world_matrix {
            if let Some(parent) = parent {
                self.world_matrix = parent.world_matrix.mul_mat4(&self.local_matrix);
            } else {
                self.world_matrix = self.local_matrix.clone();
            }
        }
    }

    pub fn get_raw_world_matrix(&self) -> [f32; 16] {
        self.world_matrix.to_cols_array()
    }
}
