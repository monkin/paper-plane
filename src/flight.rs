use crate::camera::Camera;
use crate::orientation::Orientation;
use crate::path::Path;
use crate::smooth::smooth;
use glissade::{keyframes, Animated, Easing, Keyframes, Mix};
use glm::{quat_look_at, translation, Mat4, Quat, Vec3};

#[derive(Clone, Copy, Debug, PartialEq, Mix)]
pub struct ControlPoint {
    pub plane_position: Vec3,
    pub plane_orientation: Orientation,
    pub camera_position: Vec3,
    pub camera_fov: f32,
    pub fold_phase: f32,
    pub cover_opacity: f32,
}

impl ControlPoint {
    pub fn get_camera(&self, ratio: f32) -> Camera {
        Camera {
            position: self.camera_position,
            target: Vec3::new(0.0, 0.0, 0.0),
            fov: self.camera_fov.to_radians(),
            width: ratio,
            height: 1.0,
            far: 1000.0,
            near: 0.01,
        }
    }

    pub fn get_model_matrix(&self) -> Mat4 {
        translation(&self.plane_position) * self.plane_orientation.get_matrix()
    }

    pub fn with_position(plane_position: Vec3, plane_orientation: Orientation) -> ControlPoint {
        ControlPoint {
            plane_position,
            plane_orientation,
            camera_position: DEFAULT_CAMERA_POSITION,
            camera_fov: DEFAULT_FOV,
            fold_phase: 1.0,
            cover_opacity: 0.0,
        }
    }
}

const DEFAULT_FOV: f32 = 40.0;
const DEFAULT_CAMERA_POSITION: Vec3 = Vec3::new(0.0, 0.0, -2.25);

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
            plane_orientation: Orientation::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
            camera_position: Vec3::new(0.0, 0.0, -1.0),
            camera_fov: 10.0,
            fold_phase: 0.0,
            cover_opacity: 1.0,
        })
        .ease_to(
            ControlPoint {
                plane_position: Vec3::new(0.0, 0.0, 0.0),
                plane_orientation: Orientation::new(
                    Vec3::new(0.0, 1.0, 0.0),
                    Vec3::new(0.0, 0.0, 1.0),
                ),
                camera_position: Vec3::new(0.0, 0.0, -1.0),
                camera_fov: 10.0,
                fold_phase: 0.0,
                cover_opacity: 0.0,
            },
            1.0,
            Easing::QuadraticInOut,
        )
        .ease_to(
            ControlPoint {
                plane_position: Vec3::new(0.0, 0.0, 0.0),
                plane_orientation: Orientation::new(
                    Vec3::new(0.0, 1.0, 0.0),
                    Vec3::new(0.0, 0.0, 1.0),
                ),
                camera_position: DEFAULT_CAMERA_POSITION,
                camera_fov: DEFAULT_FOV,
                fold_phase: 0.0,
                cover_opacity: 0.0,
            },
            1.0,
            Easing::QuadraticInOut,
        );

        let animation = {
            let duration = 10.0;
            let rotation_duration = 3.0;

            let fold_phase = keyframes::from::<f32, f32>(0.0).go_to(1.0, duration);
            let direction = keyframes::from::<Orientation, f32>(Orientation::new(
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 0.0, -1.0),
            ))
            .stay(duration - rotation_duration)
            .ease_to(
                Orientation::new(
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
                    cover_opacity: 0.0,
                },
            );

            animation.then(fold_animation)
        };

        let animation = animation.ease_to(
            ControlPoint::with_position(
                Vec3::new(-0.2, -0.2, -0.3),
                Orientation::new(
                    Vec3::new(0.5, 0.15, 1.0).normalize(),
                    Vec3::new(0.0, -1.0, 0.0),
                ),
            ),
            0.5,
            Easing::QuarticOut,
        );

        let animation = {
            let mut path = Path::new();

            let initial_position = Vec3::new(-0.2, -0.2, -0.3);
            let initial_direction = Vec3::new(0.5, 0.15, 1.0).normalize();

            path.go_to(initial_position);
            path.go_to(initial_position + initial_direction * 1.5);
            path.go_to(Vec3::new(1.0, -0.3, 2.5));
            path.go_to(Vec3::new(0.0, -0.5, 4.0));
            path.go_to(Vec3::new(-1.0, -0.5, 3.5));
            path.go_to(Vec3::new(-1.5, -0.2, 2.0));
            path.go_to(Vec3::new(0.0, 0.0, 0.0));

            path.go_to(Vec3::new(2.0, 0.2, 2.5));
            path.go_to(Vec3::new(0.0, 0.5, 4.0));
            path.go_to(Vec3::new(-1.5, 0.4, 3.0));
            path.go_to(Vec3::new(0.5, 0.25, -1.0));
            path.go_to(Vec3::new(2.0, 0.1, -3.0));

            let points = path.get_points();
            let points = (0..16).fold(points, |points, _| smooth(points));

            let poly = keyframes::poly(points, 10.0, Easing::Linear);

            let cover = keyframes::from(0.0).stay(9.0).go_to(1.0, 1.0);

            let path_animation = (poly, cover).map(|(point, cover_opacity)| ControlPoint {
                plane_position: point.position,
                plane_orientation: point.orientation(),
                camera_position: DEFAULT_CAMERA_POSITION,
                camera_fov: DEFAULT_FOV,
                fold_phase: 1.0,
                cover_opacity,
            });

            animation.then(path_animation)
        };

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
