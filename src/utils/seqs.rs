use num_traits::{One, Zero};
use std::{mem, ops::Add};

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
        FibonacciIter::start_at(T::zero(), T::one())
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
        let new_n = self.m + self.n;
        let new_m = mem::replace(&mut self.n, new_n);
        let cur_m = mem::replace(&mut self.m, new_m);
        Some(cur_m)
    }
}

/// Triangular number sequence iterator.
pub struct TriangularIter<T> {
    n: T,
    d: T,
}

impl<T> TriangularIter<T>
where
    T: One + Add<T, Output = T> + Clone,
{
    /// Construct a new Triangular number sequence iterator, starting at 1.
    ///
    /// The n<sup>th</sup> Triangular number is defined as the sum of the first N natural numbers.
    #[inline]
    pub fn new() -> Self {
        TriangularIter {
            n: T::one(),
            d: T::one() + T::one(),
        }
    }
}

impl<T> Default for TriangularIter<T>
where
    T: One + Add<T, Output = T> + Clone,
{
    fn default() -> Self {
        TriangularIter::new()
    }
}

impl<T> Iterator for TriangularIter<T>
where
    T: One + Add<T, Output = T> + Copy,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let new_n = self.n + self.d;
        self.d = self.d + T::one();
        Some(mem::replace(&mut self.n, new_n))
    }
}

/// Collatz sequence iterator.
pub struct CollatzIter<T> {
    n: T,
}

impl<T> CollatzIter<T> {
    #[inline]
    pub fn start_at(n: T) -> Self {
        Self { n }
    }
}

impl<T> Iterator for CollatzIter<T>
where
    T: num_integer::Integer + Copy,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let one = T::one();
        let two = one + T::one();
        let thr = two + T::one();
        let next = if self.n.is_even() {
            self.n / two
        } else {
            thr * self.n + one
        };
        Some(mem::replace(&mut self.n, next))
    }
}
