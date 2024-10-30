use crate::bit_set::BitSet;
use crate::model::transform::add_lines::AddLines;
use crate::model::transform::fold::Fold;
use crate::model::transform::shift_all::ShiftAll;
use crate::model::transform::transform_parallel::TransformParallel;
use crate::model::Model;
use glm::Vec3;

pub trait Transform {
    fn apply(&self, model: Model, t: f32) -> Model;
    fn fold(self, line: (u8, u8), points: BitSet, angle: f32) -> TransformParallel<Self, Fold>
    where
        Self: Sized,
    {
        TransformParallel::new(self, Fold::new(line, points, angle))
    }

    fn shift(self, shift: Vec3) -> TransformParallel<Self, ShiftAll>
    where
        Self: Sized,
    {
        TransformParallel::new(self, ShiftAll::new(shift))
    }

    fn add_lines(self, lines: Vec<(u8, u8)>) -> TransformParallel<Self, AddLines>
    where
        Self: Sized,
    {
        TransformParallel::new(self, AddLines::new(lines))
    }
}
