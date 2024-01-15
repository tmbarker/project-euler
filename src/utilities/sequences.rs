use num::traits::One;
use num::traits::Zero;
use std::ops::Add;

pub struct Fibonacci<T>
where
    T: Zero + One + Copy,
{
    m: T,
    n: T,
}

impl<T: Zero + One + Copy> Fibonacci<T> {
    pub fn new() -> Fibonacci<T> {
        Fibonacci::start_at(Zero::zero(), One::one())
    }

    pub fn start_at(m: T, n: T) -> Fibonacci<T> {
        Fibonacci { m, n }
    }
}

impl<T> Default for Fibonacci<T>
where
    T: Zero + One + Copy,
{
    fn default() -> Self {
        Fibonacci::new()
    }
}

impl<T: Zero + One + Add<T, Output = T> + Copy> Iterator for Fibonacci<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        *self = Fibonacci::start_at(self.n, self.m + self.n);
        Some(self.m)
    }
}
