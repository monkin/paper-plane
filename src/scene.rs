use crate::camera::Camera;
use glm::Vec3;

pub struct Scene {
    pub camera: Camera,
    pub light_position: Vec3,
}

impl Scene {
    pub fn new(camera: Camera, light_position: Vec3) -> Scene {
        Scene {
            camera,
            light_position,
        }
    }
}
