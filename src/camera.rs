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
        let v = (self.target - self.position).normalize();
        let up = Vec3::new(-v.z, -v.y, v.x);

        perspective_fov_zo(self.fov, self.width, self.height, self.near, self.far)
            * look_at(&self.position, &self.target, &up)
    }
}

impl From<Camera> for Mat4 {
    fn from(value: Camera) -> Self {
        value.get_matrix()
    }
}
