use num_traits::{One, Zero};
use std::ops::Add;

pub struct Fibonacci<T>
{
    m: T,
    n: T,
}

impl<T: Zero + One> Fibonacci<T> {
    pub fn new() -> Self {
        Fibonacci::start_at(Zero::zero(), One::one())
    }

    pub fn start_at(m: T, n: T) -> Self {
        Fibonacci { m, n }
    }
}

impl<T: Zero + One> Default for Fibonacci<T>
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
