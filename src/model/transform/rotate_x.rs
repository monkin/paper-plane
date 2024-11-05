use crate::model::transform::Transform;
use crate::model::Model;
use glm::{mat4_to_mat3, rotation, Vec3};

pub struct RotateX {
    angle: f32,
}

impl RotateX {
    pub fn new(angle: f32) -> Self {
        Self { angle }
    }
}

impl Transform for RotateX {
    fn apply(&self, model: Model, t: f32) -> Model {
        let mut vertices = model.vertices;
        let rotation = mat4_to_mat3(&rotation(self.angle * t, &Vec3::new(1.0, 0.0, 0.0)));

        for vertex in vertices.iter_mut() {
            *vertex = rotation * *vertex;
        }

        Model { vertices, ..model }
    }
}
