use glm::{rotation, translation, vec4_to_vec3, Mat4};

use crate::bit_set::BitSet;
use crate::model::transform::transform::Transform;
use crate::model::Model;

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
}

impl Transform for Fold {
    fn apply(&self, model: Model, t: f32) -> Model {
        let mut vertices = model.vertices;
        let p0 = vertices[self.line.0 as usize];
        let p1 = vertices[self.line.1 as usize];

        let transformation: Mat4 = translation(&p0)
            * rotation(self.angle * t, &(p1 - p0).normalize())
            * translation(&(-p0));

        for i in 0..vertices.len() {
            if self.points.has(i as u8) {
                vertices[i] = vec4_to_vec3(&(transformation * vertices[i].push(1.0)));
            }
        }

        Model { vertices, ..model }
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
        let model = Model {
            vertices: vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0)],
            triangles: vec![],
            lines: vec![],
        };

        let result = fold.apply(model, 1.0).vertices;
        assert_eq!(result[0], Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(result[1], Vec3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_rotate_over_origin() {
        let fold = Fold::new((0, 1), BitSet::with_bits(&[2]), std::f32::consts::PI / 2.0);

        let model = Model {
            vertices: vec![
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ],
            triangles: vec![],
            lines: vec![],
        };

        let v0 = round(fold.apply(model.clone(), 0.0).vertices[2]);
        let v1 = round(fold.apply(model.clone(), 0.5).vertices[2]);
        let v2 = round(fold.apply(model.clone(), 1.0).vertices[2]);

        assert_eq!(v0, Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(v1, Vec3::new(0.0, 0.7109375, 0.7109375));
        assert_eq!(v2, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_rotate_over_point() {
        let fold = Fold::new((0, 1), BitSet::with_bits(&[2]), std::f32::consts::PI / 2.0);
        let point = Vec3::new(1.0, 2.0, 3.0);

        let model = Model {
            vertices: vec![
                Vec3::new(0.0, 0.0, 0.0) + point,
                Vec3::new(1.0, 0.0, 0.0) + point,
                Vec3::new(0.0, 1.0, 0.0) + point,
            ],
            triangles: vec![],
            lines: vec![],
        };

        let v0 = round(fold.apply(model.clone(), 0.0).vertices[2]);
        let v1 = round(fold.apply(model.clone(), 0.5).vertices[2]);
        let v2 = round(fold.apply(model.clone(), 1.0).vertices[2]);

        assert_eq!(v0, Vec3::new(1.0, 3.0, 3.0));
        assert_eq!(v1, Vec3::new(1.0, 2.7109375, 3.7109375));
        assert_eq!(v2, Vec3::new(1.0, 2.0, 4.0));
    }
}
