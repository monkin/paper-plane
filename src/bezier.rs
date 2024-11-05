use glissade::Mix;
use std::ops::Sub;

pub struct Bezier<T>(T, T, T, T)
where
    T: Mix + Clone + Copy;

impl<T> Bezier<T>
where
    T: Mix + Clone + Copy,
{
    pub fn value_at(&self, t: f32) -> T {
        let v01 = self.0.mix(self.1, t);
        let v12 = self.1.mix(self.2, t);
        let v23 = self.2.mix(self.3, t);

        let n1 = v01.mix(v12, t);
        let n2 = v12.mix(v23, t);

        n1.mix(n2, t)
    }

    pub fn direction_at(&self, t: f32) -> T::Output
    where
        T: Sub<T>,
    {
        let v01 = self.0.mix(self.1, t);
        let v12 = self.1.mix(self.2, t);
        let v23 = self.2.mix(self.3, t);

        let n1 = v01.mix(v12, t);
        let n2 = v12.mix(v23, t);

        n2 - n1
    }
}
