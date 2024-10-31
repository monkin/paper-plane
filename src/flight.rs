use crate::camera::Camera;
use glissade::{keyframes, Animated, Easing, Keyframes, Mix};
use glm::{quat_look_at, quat_to_mat4, Mat4, Quat, Vec3};
use std::f32::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq, Mix)]
pub struct ControlPoint {
    pub plane_position: Vec3,
    pub plane_rotation: Quat,
    pub camera_position: Vec3,
    pub camera_fov: f32,
    pub fold_phase: f32,
}

impl ControlPoint {
    pub fn get_camera(&self, ratio: f32) -> Camera {
        Camera {
            position: self.camera_position,
            target: Vec3::new(0.0, 0.0, 0.0),
            fov: self.camera_fov / 180.0 * PI,
            width: 2.0,
            height: 2.0 / ratio,
            far: 100.0,
            near: 0.01,
        }
    }

    pub fn get_model_matrix(&self) -> Mat4 {
        quat_to_mat4(&self.plane_rotation)
    }
}

const DEFAULT_FOV: f32 = 43.0;
const DEFAULT_CAMERA_POSITION: Vec3 = Vec3::new(0.0, 0.0, -2.0);

fn direction_to_quat(direction: Vec3, up: Vec3) -> Quat {
    quat_look_at(&direction, &up)
}

pub struct Flight {
    animation: Box<dyn Animated<ControlPoint, f32>>,
}

impl Flight {
    pub fn new() -> Flight {
        let animation = keyframes::from(ControlPoint {
            plane_position: Vec3::new(0.0, 0.0, 0.0),
            plane_rotation: Quat::identity(),
            camera_position: Vec3::new(0.0, 0.0, -1.0),
            camera_fov: 10.0,
            fold_phase: 0.0,
        })
        .stay(0.5)
        .ease_to(
            ControlPoint {
                plane_position: Vec3::new(0.0, 0.0, 0.0),
                plane_rotation: Quat::identity(),
                camera_position: DEFAULT_CAMERA_POSITION,
                camera_fov: DEFAULT_FOV,
                fold_phase: 0.0,
            },
            1.0,
            Easing::QuadraticInOut,
        );

        let animation = {
            let duration = 10.0;
            let rotation_duration = 3.0;

            let fold_phase =
                keyframes::from::<f32, f32>(0.0).ease_to(1.0, duration, Easing::QuadraticInOut);
            let direction = keyframes::from::<Quat, f32>(Quat::identity())
                .stay(duration - rotation_duration)
                .ease_to(
                    direction_to_quat(Vec3::new(0.1, -1.0, 0.1), Vec3::new(0.0, 1.0, 0.0)),
                    rotation_duration,
                    Easing::QuadraticInOut,
                );
            let fold_animation =
                (fold_phase, direction).map(|(fold_phase, plane_rotation)| ControlPoint {
                    fold_phase,
                    plane_rotation,
                    camera_position: DEFAULT_CAMERA_POSITION,
                    camera_fov: DEFAULT_FOV,
                    plane_position: Vec3::new(0.0, 0.0, 0.0),
                });

            animation.then(fold_animation)
        };

        let duration = animation.duration();
        let animation = animation.stay(2.0).scale(1.0 / duration).repeat().run(0.0);

        Flight {
            animation: Box::new(animation),
        }
    }

    pub fn get(&self, t: f32) -> ControlPoint {
        self.animation.get(t)
    }
}
