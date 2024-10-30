use crate::model::transform::Transform;
use crate::model::Model;
use glm::Vec3;

pub struct ShiftAll {
    shift: Vec3,
}

impl ShiftAll {
    pub fn new(shift: Vec3) -> Self {
        Self { shift }
    }
}

impl Transform for ShiftAll {
    fn apply(&self, model: Model, t: f32) -> Model {
        let shift = self.shift * t;
        let mut vertices = model.vertices;
        for vertex in vertices.iter_mut() {
            *vertex += shift;
        }
        Model { vertices, ..model }
    }
}
