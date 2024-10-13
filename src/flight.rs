use glm::{Mat4, Vec3};
use num_traits::One;

#[derive(Clone, Copy, Debug, PartialEq)]
struct ControlPoint {
    position: Vec3,
    time: f32,
}

pub struct Flight {}

impl Flight {
    pub fn get_matrix(_time: f32) -> Mat4 {
        Mat4::one()
    }
}
