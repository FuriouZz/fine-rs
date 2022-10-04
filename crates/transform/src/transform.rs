use math::{Mat3, Quat, Vec3};

#[derive(Debug, Clone)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

impl Transform {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn multiply_transform(&self, transform: &Self) -> Self {
        let translation = self.multiply_vec3(transform.translation);
        let rotation = self.rotation * transform.rotation;
        let scale = self.scale * transform.scale;
        Self {
            translation,
            scale,
            rotation,
        }
    }

    pub fn multiply_vec3(&self, mut value: Vec3) -> Vec3 {
        value = self.rotation * value;
        value = self.scale * value;
        value += self.translation;
        value
    }

    pub fn rotate(&mut self, rotation: Quat) {
        self.rotation = self.rotation * rotation;
    }

    pub fn rotate_x(&mut self, angle: f32) {
        self.rotation = self.rotation * Quat::from_rotation_x(angle);
    }

    pub fn rotate_y(&mut self, angle: f32) {
        self.rotation = self.rotation * Quat::from_rotation_y(angle);
    }

    pub fn rotate_z(&mut self, angle: f32) {
        self.rotation = self.rotation * Quat::from_rotation_z(angle);
    }

    pub fn look_at(&mut self, target: Vec3, up: Vec3) {
        let forward = Vec3::normalize(self.translation - target);
        let right = up.cross(forward).normalize();
        let up = forward.cross(right);
        self.rotation = Quat::from_mat3(&Mat3::from_cols(right, up, forward));
    }
}
