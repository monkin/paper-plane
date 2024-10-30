use crate::model::{Model, Transform};

pub struct AddLines {
    lines: Vec<(u8, u8)>,
}

impl AddLines {
    pub fn new(lines: Vec<(u8, u8)>) -> Self {
        Self { lines }
    }
}

impl Transform for AddLines {
    fn apply(&self, model: Model, t: f32) -> Model {
        let mut lines = model.lines;
        lines.extend(self.lines.iter().copied().map(|(a, b)| (a, b, t)));
        Model { lines, ..model }
    }
}
