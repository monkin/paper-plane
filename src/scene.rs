use crate::camera::Camera;
use glm::{Mat4, Vec3};

pub struct Scene {
    pub camera: Camera,
    pub light_position: Vec3,
    pub model_matrix: Mat4,
}
