use glissade::Mix;
use glm::{quat_inverse, quat_look_at, quat_slerp, quat_to_mat4, rotate, scale, Mat4, Quat, Vec3};
use std::f32::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Orientation {
    quat: Quat,
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation {
            quat: Quat::identity(),
        }
    }
}

impl Orientation {
    pub fn new(direction: Vec3, up: Vec3) -> Orientation {
        Orientation {
            quat: quat_inverse(&quat_look_at(&direction, &up)),
        }
    }

    pub fn get_matrix(&self) -> Mat4 {
        scale(
            &rotate(
                &quat_to_mat4(&self.quat),
                PI * 0.5,
                &Vec3::new(1.0, 0.0, 0.0),
            ),
            &Vec3::new(1.0, 1.0, 1.0),
        )
    }
}

impl Mix for Orientation {
    fn mix(self, other: Self, t: f32) -> Self {
        Orientation {
            quat: quat_slerp(&self.quat, &other.quat, t),
        }
    }
}
