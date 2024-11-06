use crate::bit_set::BitSet;
use crate::model::transform::add_lines::AddLines;
use crate::model::transform::shift::Shift;
use crate::model::transform::shift_all::ShiftAll;
use crate::model::transform::transform_parallel::TransformParallel;
use crate::model::transform::RotateX;
use crate::model::Model;
use glm::Vec3;

pub trait Transform {
    fn apply(&self, model: Model, t: f32) -> Model;

    fn shift_all(self, shift: Vec3) -> TransformParallel<Self, ShiftAll>
    where
        Self: Sized,
    {
        TransformParallel::new(self, ShiftAll::new(shift))
    }

    fn shift(self, shift: Vec3, points: BitSet) -> TransformParallel<Self, Shift>
    where
        Self: Sized,
    {
        TransformParallel::new(self, Shift::new(shift, points))
    }

    fn rotate_x(self, angle: f32) -> TransformParallel<Self, RotateX>
    where
        Self: Sized,
    {
        TransformParallel::new(self, RotateX::new(angle))
    }

    fn add_lines(self, lines: Vec<(u8, u8)>) -> TransformParallel<Self, AddLines>
    where
        Self: Sized,
    {
        TransformParallel::new(self, AddLines::new(lines))
    }
}
