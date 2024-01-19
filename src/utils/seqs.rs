use num_traits::{One, Zero};
use std::ops::{Add, Div, Mul};

/// Fibonacci sequence iterator.
pub struct FibonacciIter<T> {
    m: T,
    n: T,
}

impl<T: Zero + One> FibonacciIter<T> {
    /// Construct a new Fibonacci sequence iterator, starting at the canonical values.
    ///
    /// The Fibonacci sequence is defined as follows:
    /// * F(0) = 0
    /// * F(1) = 1
    /// * F(n) = F(n-1) + F(n-2)
    #[inline]
    pub fn new() -> Self {
        FibonacciIter::start_at(Zero::zero(), One::one())
    }

    /// Construct a new Fibonacci sequence iterator, starting at the specified values.
    #[inline]
    pub fn start_at(m: T, n: T) -> Self {
        FibonacciIter { m, n }
    }
}

impl<T: Zero + One> Default for FibonacciIter<T> {
    fn default() -> Self {
        FibonacciIter::new()
    }
}

impl<T: Zero + One + Add<T, Output = T> + Copy> Iterator for FibonacciIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        *self = FibonacciIter::start_at(self.n, self.m + self.n);
        Some(self.m)
    }
}

/// Triangular number sequence iterator.
pub struct TriangularIter<T> {
    n: T,
}

impl<T: Zero> TriangularIter<T> {
    /// Construct a new Triangular number sequence iterator, starting at 1.
    ///
    /// The n<sup>th</sup> Triangular number is defined as the sum of the first N natural numbers.
    #[inline]
    pub fn new() -> Self {
        TriangularIter::start_at(Zero::zero())
    }

    /// Construct a new Triangular number sequence iterator, starting at the specified values.
    #[inline]
    pub fn start_at(n: T) -> Self {
        TriangularIter { n }
    }
}

impl<T: Zero> Default for TriangularIter<T> {
    fn default() -> Self {
        TriangularIter::new()
    }
}

impl<T> Iterator for TriangularIter<T>
where
    T: Zero + One + Add<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T> + Copy,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let n = self.n;
        let v = n * (n + T::one()) / (T::one() + T::one());
        *self = TriangularIter::start_at(self.n + T::one());
        Some(v)
    }
}
