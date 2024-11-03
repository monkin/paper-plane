use glissade::Mix;
use glm::{quat_inverse, quat_look_at, quat_slerp, quat_to_mat4, rotate, scale, Mat4, Quat, Vec3};
use std::f32::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlaneOrientation {
    quat: Quat,
}

impl Default for PlaneOrientation {
    fn default() -> Self {
        PlaneOrientation {
            quat: Quat::identity(),
        }
    }
}

impl PlaneOrientation {
    pub fn new(direction: Vec3, up: Vec3) -> PlaneOrientation {
        PlaneOrientation {
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

impl Mix for PlaneOrientation {
    fn mix(self, other: Self, t: f32) -> Self {
        PlaneOrientation {
            quat: quat_slerp(&self.quat, &other.quat, t),
        }
    }
}
