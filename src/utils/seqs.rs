use num_traits::{One, Zero};
use std::ops::Add;

/// Fibonacci sequence iterator.
pub struct Fibonacci<T> {
    m: T,
    n: T,
}

impl<T: Zero + One> Fibonacci<T> {
    /// Construct a new Fibonacci sequence iterator, starting at the canonical values.
    ///
    /// The Fibonacci sequence is defined as follows:
    /// * F(0) = 0
    /// * F(1) = 1
    /// * F(n) = F(n-1) + F(n-2)
    #[inline]
    pub fn new() -> Self {
        Fibonacci::start_at(Zero::zero(), One::one())
    }

    /// Construct a new Fibonacci sequence iterator, starting at the specified values.
    #[inline]
    pub fn start_at(m: T, n: T) -> Self {
        Fibonacci { m, n }
    }
}

impl<T: Zero + One> Default for Fibonacci<T> {
    fn default() -> Self {
        Fibonacci::new()
    }
}

impl<T: Zero + One + Add<T, Output = T> + Copy> Iterator for Fibonacci<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        *self = Fibonacci::start_at(self.n, self.m + self.n);
        Some(self.m)
    }
}
