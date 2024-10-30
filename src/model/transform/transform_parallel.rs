use crate::model::transform::Transform;
use crate::model::Model;

pub struct TransformParallel<T1: Transform, T2: Transform> {
    t1: T1,
    t2: T2,
}

impl<T1: Transform, T2: Transform> TransformParallel<T1, T2> {
    pub fn new(t1: T1, t2: T2) -> Self {
        Self { t1, t2 }
    }
}

impl<T1: Transform, T2: Transform> Transform for TransformParallel<T1, T2> {
    fn apply(&self, model: Model, t: f32) -> Model {
        let model = self.t1.apply(model, t);
        self.t2.apply(model, t)
    }
}
