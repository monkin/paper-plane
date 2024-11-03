use crate::camera::Camera;
use crate::plane_orientation::PlaneOrientation;
use glissade::{keyframes, Animated, Easing, Keyframes, Mix};
use glm::{quat_look_at, translation, Mat4, Quat, Vec3};

#[derive(Clone, Copy, Debug, PartialEq, Mix)]
pub struct ControlPoint {
    pub plane_position: Vec3,
    pub plane_orientation: PlaneOrientation,
    pub camera_position: Vec3,
    pub camera_fov: f32,
    pub fold_phase: f32,
}

impl ControlPoint {
    pub fn get_camera(&self, ratio: f32) -> Camera {
        Camera {
            position: self.camera_position,
            target: Vec3::new(0.0, 0.0, 0.0),
            fov: self.camera_fov.to_radians(),
            width: ratio,
            height: 1.0,
            far: 100.0,
            near: 0.01,
        }
    }

    pub fn get_model_matrix(&self) -> Mat4 {
        translation(&self.plane_position) * self.plane_orientation.get_matrix()
    }
}

const DEFAULT_FOV: f32 = 40.0;
const DEFAULT_CAMERA_POSITION: Vec3 = Vec3::new(0.0, 0.0, -2.33);

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
            plane_orientation: PlaneOrientation::new(
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            ),
            camera_position: Vec3::new(0.0, 0.0, -1.0),
            camera_fov: 10.0,
            fold_phase: 0.0,
        })
        .stay(0.5)
        .ease_to(
            ControlPoint {
                plane_position: Vec3::new(0.0, 0.0, 0.0),
                plane_orientation: PlaneOrientation::new(
                    Vec3::new(0.0, 1.0, 0.0),
                    Vec3::new(0.0, 0.0, 1.0),
                ),
                camera_position: DEFAULT_CAMERA_POSITION,
                camera_fov: DEFAULT_FOV,
                fold_phase: 0.0,
            },
            1.0,
            Easing::QuadraticInOut,
        );

        let animation = {
            let duration = 10.0;
            let rotation_duration = 4.0;

            let fold_phase = keyframes::from::<f32, f32>(0.0).go_to(1.0, duration);
            let direction = keyframes::from::<PlaneOrientation, f32>(PlaneOrientation::new(
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 0.0, -1.0),
            ))
            .stay(duration - rotation_duration)
            .ease_to(
                PlaneOrientation::new(
                    Vec3::new(0.5, 0.0, 1.0).normalize(),
                    Vec3::new(0.0, -1.0, 0.0),
                ),
                rotation_duration,
                Easing::QuadraticInOut,
            );
            let plane_position =
                keyframes::stay(Vec3::new(0.0, 0.0, 0.0), duration - rotation_duration).ease_to(
                    Vec3::new(-0.15, -0.1, -0.1),
                    rotation_duration,
                    Easing::QuadraticInOut,
                );
            let fold_animation = (fold_phase, direction, plane_position).map(
                |(fold_phase, plane_orientation, plane_position)| ControlPoint {
                    fold_phase,
                    plane_orientation,
                    camera_position: DEFAULT_CAMERA_POSITION,
                    camera_fov: DEFAULT_FOV,
                    plane_position,
                },
            );

            animation.then(fold_animation)
        };

        let animation = animation.stay(2.0);

        let duration = animation.duration();
        let animation = animation.scale(1.0 / duration).repeat().run(0.0);

        Flight {
            animation: Box::new(animation),
        }
    }

    pub fn get(&self, t: f32) -> ControlPoint {
        self.animation.get(t)
    }
}
