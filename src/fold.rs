use glm::{rotation, translation, vec4_to_vec3, Mat4, Vec3};

use crate::bit_set::BitSet;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fold {
    line: (u8, u8),
    points: BitSet,
    angle: f32,
}

impl Fold {
    pub const fn new(line: (u8, u8), points: BitSet, angle: f32) -> Fold {
        Fold {
            line,
            points,
            angle,
        }
    }

    pub fn apply(&self, points: &[Vec3], t: f32) -> Vec<Vec3> {
        let p0 = points[self.line.0 as usize];
        let p1 = points[self.line.1 as usize];

        let transformation: Mat4 = translation(&p0)
            * rotation(self.angle * t, &(p1 - p0).normalize())
            * translation(&(-p0));

        points
            .iter()
            .copied()
            .enumerate()
            .map(|(i, p)| {
                if self.points.has(i as u8) {
                    vec4_to_vec3(&(transformation * p.push(1.0)))
                } else {
                    p
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round(v: Vec3) -> Vec3 {
        Vec3::new(
            (v.x * 128.0).round() / 128.0,
            (v.y * 128.0).round() / 128.0,
            (v.z * 128.0).round() / 128.0,
        )
    }

    #[test]
    fn test_stay_in_place() {
        let fold = Fold::new(
            (0, 1),
            BitSet::with_bits(&[0, 1]),
            std::f32::consts::PI / 2.0,
        );
        let points = vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0)];

        let result = fold.apply(&points, 1.0);
        assert_eq!(result[0], Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(result[1], Vec3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_rotate_over_origin() {
        let fold = Fold::new((0, 1), BitSet::with_bits(&[2]), std::f32::consts::PI / 2.0);
        let points = vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ];

        let v0 = round(fold.apply(&points, 0.0)[2]);
        let v1 = round(fold.apply(&points, 0.5)[2]);
        let v2 = round(fold.apply(&points, 1.0)[2]);

        assert_eq!(v0, Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(v1, Vec3::new(0.0, 0.703125, 0.703125));
        assert_eq!(v2, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_rotate_over_point() {
        let fold = Fold::new((0, 1), BitSet::with_bits(&[2]), std::f32::consts::PI / 2.0);
        let point = Vec3::new(1.0, 2.0, 3.0);

        let points = vec![
            Vec3::new(0.0, 0.0, 0.0) + point,
            Vec3::new(1.0, 0.0, 0.0) + point,
            Vec3::new(0.0, 1.0, 0.0) + point,
        ];

        let v0 = round(fold.apply(&points, 0.0)[2]);
        let v1 = round(fold.apply(&points, 0.5)[2]);
        let v2 = round(fold.apply(&points, 1.0)[2]);

        assert_eq!(v0, Vec3::new(1.0, 3.0, 3.0));
        assert_eq!(v1, Vec3::new(1.0, 2.7109375, 3.7109375));
        assert_eq!(v2, Vec3::new(1.0, 2.0, 4.0));
    }
}
