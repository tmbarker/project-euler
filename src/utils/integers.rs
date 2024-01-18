use num_traits::{FromPrimitive, One, ToPrimitive, Zero};

/// Super trait extension of num_integer::Integer which is useful for Project Euler problems
pub trait Integer: num_integer::Integer + Clone + FromPrimitive + ToPrimitive {
    #[inline]
    fn into_digits(self, radix: Self) -> Digits<Self> {
        Digits::new(self, radix)
    }

    #[inline]
    fn to_digits(&self, radix: Self) -> Digits<Self> {
        Digits::new(self.clone(), radix)
    }

    /// Returns `true` if the number is palindromic.
    fn is_palindromic(&self, radix: Self) -> bool {
        let mut digits = self.to_digits(radix);
        loop {
            let front = digits.next();
            let back = digits.next_back();
            if front.is_none() || back.is_none() {
                return true;
            }
            if front != back {
                return false;
            }
        }
    }
}

macro_rules! integer_trait_impl {
    ($($t:ty)*) => ($(
        impl Integer for $t {}
    )*)
}

integer_trait_impl!(i8 i16 i32 i64 isize u8 u16 u32 u64 usize);

/// An iterator which yields the digits of an integer.
#[derive(Clone)]
pub struct Digits<T> {
    /// The value of the integer.
    num: T,
    /// The base of the integer representation.
    radix: T,
    /// The order of magnitude of the integer, given the radix.
    order: T,
}

impl<T: num_integer::Integer + Clone> Digits<T> {
    fn new(num: T, radix: T) -> Self {
        let mut order: T;
        if num.is_zero() {
            order = Zero::zero();
        } else {
            order = One::one();
            let mut prod = order.clone() * radix.clone();
            while prod <= num {
                order = prod;
                prod = order.clone() * radix.clone()
            }
        }

        Digits { num, radix, order }
    }
}

impl<T: num_integer::Integer + Clone> Iterator for Digits<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.order.is_zero() {
            return None;
        }

        let (d, r) = self.num.div_rem(&self.radix);
        self.num = d;
        self.order = self.order.clone() / self.radix.clone();
        Some(r)
    }
}

impl<T: num_integer::Integer + Clone> DoubleEndedIterator for Digits<T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.order.is_zero() {
            return None;
        }

        let (d, r) = self.num.div_rem(&self.order);
        self.num = r;
        self.order = self.order.clone() / self.radix.clone();
        Some(d)
    }
}
