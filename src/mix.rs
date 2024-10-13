use std::ops::{Add, Mul, Sub};

use glm::TVec;

pub trait Mix {
    type Factor;

    fn mix(self, other: Self, t: Self::Factor) -> Self;
}

impl Mix for f32 {
    type Factor = f32;

    fn mix(self, other: Self, t: Self::Factor) -> Self {
        self + (other - self) * t
    }
}

impl<T, const R: usize> Mix for TVec<T, R>
where
    Self: Copy,
    TVec<T, R>: Sub<TVec<T, R>, Output = TVec<T, R>>
        + Add<TVec<T, R>, Output = TVec<T, R>>
        + Mul<T, Output = TVec<T, R>>,
{
    type Factor = T;

    fn mix(self, other: Self, t: Self::Factor) -> Self {
        self + (other - self) * t
    }
}
