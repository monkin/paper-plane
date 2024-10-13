use glm::{look_at, perspective_fov_zo, Mat4, Vec3};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub fov: f32,
    pub width: f32,
    pub height: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn get_matrix(&self) -> Mat4 {
        perspective_fov_zo(self.fov, self.width, self.height, self.near, self.far)
            * look_at(&self.position, &self.target, &Vec3::new(0.0, -1.0, 0.0))
    }
}

impl From<Camera> for Mat4 {
    fn from(value: Camera) -> Self {
        value.get_matrix()
    }
}
