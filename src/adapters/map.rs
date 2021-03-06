use crate::Uncertain;
use rand::Rng;

pub struct Map<U, F> {
    uncertain: U,
    func: F,
}

impl<T, U, F> Map<U, F>
where
    U: Uncertain,
    F: Fn(U::Value) -> T,
{
    pub fn new(uncertain: U, func: F) -> Self {
        Self { uncertain, func }
    }
}

impl<T, U, F> Uncertain for Map<U, F>
where
    U: Uncertain,
    F: Fn(U::Value) -> T,
{
    type Value = T;

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R, epoch: usize) -> Self::Value {
        let v = self.uncertain.sample(rng, epoch);
        (self.func)(v)
    }
}
