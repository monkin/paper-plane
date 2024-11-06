use crate::model::model::Model;
use crate::model::Transform;
use glissade::Easing;

pub trait AnimatedModel {
    fn duration(&self) -> f32;
    fn get_model(&self, time: f32) -> Model;

    fn animate<T>(self, duration: f32, transformation: T) -> ModelTransformation<Self, T>
    where
        T: Transform,
        Self: Sized,
    {
        ModelTransformation::new(self, transformation, duration)
    }
}

impl AnimatedModel for Model {
    fn duration(&self) -> f32 {
        0.0
    }

    fn get_model(&self, _time: f32) -> Model {
        self.clone()
    }
}

pub struct ModelTransformation<M: AnimatedModel, T: Transform> {
    model: M,
    input: Model,
    transformation: T,
    duration: f32,
}

impl<M: AnimatedModel, T: Transform> ModelTransformation<M, T> {
    pub fn new(model: M, transformation: T, duration: f32) -> Self {
        Self {
            input: model.get_model(model.duration()),
            model,
            transformation,
            duration,
        }
    }
}

impl<M: AnimatedModel, T: Transform> AnimatedModel for ModelTransformation<M, T> {
    fn duration(&self) -> f32 {
        self.model.duration() + self.duration
    }

    fn get_model(&self, time: f32) -> Model {
        if time < self.model.duration() {
            self.model.get_model(time)
        } else {
            let t = (time - self.model.duration()) / self.duration;
            let t = t.clamp(0.0, 1.0);
            let t = Easing::QuadraticInOut.ease(t);
            self.transformation.apply(self.input.clone(), t)
        }
    }
}
