use num_traits::{FromPrimitive, One, ToPrimitive, Zero};

/// Super trait extension of `num_integer::Integer`, which is useful for Project Euler problems.
pub trait Integer: num_integer::Integer + Clone + FromPrimitive + ToPrimitive {
    /// Creates an Iterator that enumerates each digit, starting with the least significant digit.
    #[inline]
    fn into_digits(self, radix: Self) -> Digits<Self> {
        Digits::new(self, radix)
    }

    /// Creates an Iterator that enumerates each digit, starting with the least significant digit.
    #[inline]
    fn to_digits(&self, radix: Self) -> Digits<Self> {
        Digits::new(self.clone(), radix)
    }

    /// Constructs an integer from a digit Iterator. Note that the Iterator should yield the
    /// digits from least significant to most significant.
    #[inline]
    fn from_digits<T: Iterator<Item = Self>>(digits: T, radix: Self) -> Self {
        let mut result = Self::zero();
        let mut order = Self::one();
        for d in digits {
            result = result + order.clone() * d;
            order = order * radix.clone();
        }
        result
    }

    /// Returns `true` if the number is unchanged when its digits are reversed.
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

    /// Returns the factorial (!) of the number, which is equal to the product of all
    /// positive integers less than or equal to the given positive integer.
    fn factorial(&self) -> Self {
        assert!(*self >= Zero::zero());

        let mut p: Self = One::one();
        let mut i: Self = One::one();
        while i <= *self {
            p = p * i.clone();
            i = i + One::one();
        }
        p
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
    /// The order of magnitude of the digit, given the radix.
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

#[cfg(test)]
mod tests {
    use super::Integer;

    #[test]
    fn digits_iter() {
        const NUMBER: usize = 1234;
        const RADIX: usize = 10;

        let mut f_iter = NUMBER.to_digits(RADIX);
        assert_eq!(f_iter.next(), Some(4));
        assert_eq!(f_iter.next(), Some(3));
        assert_eq!(f_iter.next(), Some(2));
        assert_eq!(f_iter.next(), Some(1));

        let mut r_iter = NUMBER.to_digits(RADIX);
        assert_eq!(r_iter.next_back(), Some(1));
        assert_eq!(r_iter.next_back(), Some(2));
        assert_eq!(r_iter.next_back(), Some(3));
        assert_eq!(r_iter.next_back(), Some(4));
    }

    #[test]
    fn is_palindromic() {
        const RADIX: usize = 10;
        assert!(0.is_palindromic(RADIX));
        assert!(1.is_palindromic(RADIX));
        assert!(11.is_palindromic(RADIX));
        assert!(121.is_palindromic(RADIX));
        assert!(!10.is_palindromic(RADIX));
        assert!(!100.is_palindromic(RADIX));
        assert!(!122.is_palindromic(RADIX));
    }

    #[test]
    fn factorial() {
        assert_eq!(1, 0.factorial());
        assert_eq!(1, 1.factorial());
        assert_eq!(2, 2.factorial());
        assert_eq!(6, 3.factorial());
        assert_eq!(3628800, 10.factorial());
    }
}
