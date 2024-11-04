use crate::orientation::Orientation;
use glissade::{Distance, Mix};
use glm::{distance, lerp, slerp, Vec3};
use std::iter::once;

pub struct Path {
    points: Vec<Vec3>,
}

#[derive(Clone, Copy, Debug)]
pub struct PathPoint {
    pub position: Vec3,
    pub direction: Vec3,
    pub up: Vec3,
}

impl Distance for PathPoint {
    fn distance(self, other: Self) -> f32 {
        distance(&self.position, &other.position)
    }
}

impl Mix for PathPoint {
    fn mix(self, other: Self, t: f32) -> Self {
        PathPoint {
            position: self.position.mix(other.position, t),
            direction: slerp(&self.direction, &other.direction, t),
            up: slerp(&self.up, &other.up, t),
        }
    }
}

impl PathPoint {
    pub fn orientation(&self) -> Orientation {
        Orientation::new(self.direction, self.up)
    }
}

const BEZIER_STEPS: usize = 16;

impl Path {
    pub fn new() -> Self {
        Self {
            points: Default::default(),
        }
    }

    pub fn go_to(&mut self, target: Vec3) -> &mut Self {
        self.points.push(target);
        self
    }

    fn direction_at(&self, index: usize) -> Vec3 {
        let length = self.points.len();
        if length < 2 {
            Vec3::new(0.0, 0.0, 0.0)
        } else if index == 0 {
            self.points[1] - self.points[0]
        } else if index == self.points.len() - 1 {
            self.points[length - 1] - self.points[length - 2]
        } else {
            (self.points[index + 1] - self.points[index - 1]) * 0.5
        }
        .normalize()
    }

    fn up_at(&self, index: usize) -> Vec3 {
        let length = self.points.len();
        if index == 0 || index == length - 1 || length < 2 {
            Vec3::new(0.0, -1.0, 0.0)
        } else {
            let p1 = self.points[index - 1];
            let p2 = self.points[index];
            let p3 = self.points[index + 1];

            let m = (p3 + p1) * 0.5;
            let d = (m - p2) * (2.0 / distance(&p3, &p1));

            (Vec3::new(0.0, -1.0, 0.0) - d).normalize()
        }
    }

    fn point_at(&self, index: usize) -> PathPoint {
        PathPoint {
            position: self.points[index],
            direction: self.direction_at(index),
            up: self.up_at(index),
        }
    }

    pub(crate) fn get_points(&self) -> Vec<PathPoint> {
        once(self.point_at(0))
            .chain(
                (0..self.points.len() - 1)
                    .map(|i| (self.point_at(i), self.point_at(i + 1)))
                    .flat_map(|(p1, p4)| {
                        let d = distance(&p1.position, &p4.position) * 0.33;
                        let p2 = PathPoint {
                            position: p1.position + p1.direction * d,
                            direction: lerp(&p1.direction, &p4.direction, 0.33).normalize(),
                            up: slerp(&p1.up, &p4.up, 0.33),
                        };
                        let p3 = PathPoint {
                            position: p4.position - p4.direction * d,
                            direction: lerp(&p1.direction, &p4.direction, 0.66).normalize(),
                            up: slerp(&p1.up, &p4.up, 0.66),
                        };

                        (1..=BEZIER_STEPS).map(move |j| {
                            let t = j as f32 / BEZIER_STEPS as f32;
                            let m12 = p1.mix(p2, t);
                            let m23 = p2.mix(p3, t);
                            let m34 = p3.mix(p4, t);

                            let m123 = m12.mix(m23, t);
                            let m234 = m23.mix(m34, t);

                            m123.mix(m234, t)
                        })
                    }),
            )
            .collect()
    }
}
