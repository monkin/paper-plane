use crate::model::{Model, Transform};

pub struct Stay {}

impl Stay {
    pub fn new() -> Self {
        Self {}
    }
}

impl Transform for Stay {
    fn apply(&self, model: Model, _t: f32) -> Model {
        model
    }
}
