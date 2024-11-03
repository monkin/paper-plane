use crate::bit_set::BitSet;
use crate::model::{Model, Transform};
use glm::Vec3;

pub struct Shift {
    shift: Vec3,
    points: BitSet,
}

impl Shift {
    pub fn new(shift: Vec3, points: BitSet) -> Self {
        Self { shift, points }
    }
}

impl Transform for Shift {
    fn apply(&self, mut model: Model, t: f32) -> Model {
        let shift = self.shift * t;
        let vertices = &mut model.vertices;
        for (i, vertex) in vertices.iter_mut().enumerate() {
            if self.points.has(i as u8) {
                *vertex += shift;
            }
        }
        model
    }
}
