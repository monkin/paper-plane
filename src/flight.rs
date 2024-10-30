use glissade::{keyframes, Animated, Keyframes, Mix};
use glm::{quat_look_at, Quat, Vec3};

#[derive(Clone, Copy, Debug, PartialEq, Mix)]
struct ControlPoint {
    plane_position: Vec3,
    plane_rotation: Quat,
    camera_position: Vec3,
    fold_phase: f32,
}

fn direction_to_quat(direction: Vec3) -> Quat {
    quat_look_at(&direction, &Vec3::new(0.0, 1.0, 0.0))
}

pub struct Flight {
    animation: Box<dyn Animated<ControlPoint, f32>>,
}

impl Flight {
    pub fn new() -> Flight {
        let animation = keyframes::from(ControlPoint {
            plane_position: Vec3::new(0.0, 0.0, 0.0),
            plane_rotation: Quat::identity(),
            camera_position: Vec3::new(0.0, 0.0, -0.5),
            fold_phase: 0.0,
        })
        .go_to(
            ControlPoint {
                plane_position: Vec3::new(0.0, 0.0, 0.0),
                plane_rotation: Quat::identity(),
                camera_position: Vec3::new(0.0, 0.0, -3.0),
                fold_phase: 0.0,
            },
            1.0,
        );

        let animation = {
            let duration = 10.0;
            let rotation_duration = 3.0;

            let fold_phase = keyframes::from::<f32, f32>(0.0).go_to(1.0, duration);
            let direction = keyframes::from::<Quat, f32>(Quat::identity())
                .stay(duration - rotation_duration)
                .go_to(
                    direction_to_quat(Vec3::new(0.0, 0.1, 1.0)),
                    rotation_duration,
                );
            (fold_phase, direction).map(|v| 1.0)
        };

        let animation = animation.scale_to(1.0).repeat().run(0.0);

        Flight {
            animation: Box::new(animation),
        }
    }

    pub fn get(&self, t: f32) -> ControlPoint {
        self.animation.get(t)
    }
}
