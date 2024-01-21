use num_integer::Integer;
use num_traits::{FromPrimitive, One, Zero};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::{cell::RefCell, collections::HashMap, hash::Hash, mem, rc::Rc};

const INITIAL_CAPACITY: usize = 10000;
const SEED_PRIMES: &[u64] = &[
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547,
    557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
    661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797,
    809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929,
    937, 941, 947, 953, 967, 971, 977, 983, 991, 997,
];

/// Prime number generator.
#[derive(Clone)]
pub struct PrimeSeq {
    data: Rc<RefCell<PrimeInner>>,
}

impl PrimeSeq {
    /// Construct a new prime number generator, containing only the first and second prime.
    #[inline]
    pub fn new() -> Self {
        Self::from_inner(PrimeInner::new())
    }

    /// Construct a new prime number generator, seeded with a set of small primes.
    #[inline]
    pub fn seeded(capacity: usize) -> Self {
        Self::from_inner(PrimeInner::seeded(capacity))
    }

    /// Compute the n<sup>th</sup> prime, where n is zero-indexed.
    #[inline]
    pub fn nth(&self, n: usize) -> u64 {
        self.data.borrow_mut().compute_nth(n)
    }

    /// Compute if a number is prime.
    #[inline]
    pub fn is_prime(&self, n: u64) -> bool {
        self.data.borrow_mut().check_prime(n)
    }

    /// Get an iterator which yields primes in ascending order.
    pub fn iter(&self) -> PrimeSeqIter {
        PrimeSeqIter {
            idx: 0,
            data: self.data.clone(),
        }
    }

    /// Calculate the combination nCr.
    pub fn combinations(&self, n: u64, k: u64) -> u64 {
        let mut factorized = Factorized::<u64>::new(self);
        for i in (n - k + 1)..=n {
            factorized.mul(i);
        }
        for i in 1..=k {
            factorized.div(i);
        }
        factorized.into_integer()
    }

    #[inline]
    fn from_inner(gen: PrimeInner) -> Self {
        PrimeSeq {
            data: Rc::new(RefCell::new(gen)),
        }
    }
}

/// Prime number sequence iterator.
pub struct PrimeSeqIter {
    idx: usize,
    data: Rc<RefCell<PrimeInner>>,
}

impl Iterator for PrimeSeqIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let p = self.data.borrow_mut().compute_nth(self.idx);
        self.idx += 1;
        Some(p)
    }
}

/// A factor of a number, specifying the base and exponent of the factor.
pub struct Factor<T> {
    pub base: T,
    pub exp: i32,
}

/// An iterator which lazily yields the prime factors of a number.
pub struct FactorsIter<T> {
    num: T,
    primes: PrimeSeqIter,
}

/// Numbers which can be factorized.
pub trait Factorize: Integer + FromPrimitive + Clone {
    /// An iterator yielding all prime factors in ascending order.
    fn factorize(&self, ps: &PrimeSeq) -> FactorsIter<Self>;

    /// Compute the number of divisors of the the number.
    fn num_divisors(&self, ps: &PrimeSeq) -> u64 {
        if self.is_zero() {
            return Zero::zero();
        }

        // This is an implementation of the Tau function (divisor function)
        self.factorize(ps)
            .map(|factor| (factor.exp as u64) + 1)
            .product()
    }
}

/// Implement the Factorize trait for an unsigned integer type.
macro_rules! factorize_trait_impl_unsigned {
    ($($t:ty)*) => ($(
        impl Factorize for $t {
            fn factorize(&self, ps: &PrimeSeq) -> FactorsIter<$t> {
                FactorsIter { num: *self, primes: ps.iter() }
            }
        }
    )*)
}

/// Implement the Factorize trait for a signed integer type.
macro_rules! factorize_trait_impl_signed {
    ($($t:ty)*) => ($(
        impl Factorize for $t {
            fn factorize(&self, ps: &PrimeSeq) -> FactorsIter<$t> {
                if *self < 0 {
                    FactorsIter { num: -*self, primes: ps.iter() }
                } else {
                    FactorsIter { num: *self, primes: ps.iter() }
                }
            }
        }
   )*)
}

factorize_trait_impl_unsigned!(usize u8 u16 u32 u64);
factorize_trait_impl_signed!(isize i8 i16 i32 i64);

impl<T: Integer + FromPrimitive + Clone> Iterator for FactorsIter<T> {
    type Item = Factor<T>;

    fn next(&mut self) -> Option<Factor<T>> {
        if self.num <= One::one() {
            return None;
        }

        for p in &mut self.primes {
            let p: T = FromPrimitive::from_u64(p).unwrap();
            if p.clone() * p.clone() > self.num {
                let n = mem::replace(&mut self.num, One::one());
                return Some(Factor { base: n, exp: 1 });
            }

            if self.num.is_multiple_of(&p) {
                let mut exp = 1;
                self.num = self.num.clone() / p.clone();
                while self.num.is_multiple_of(&p) {
                    exp += 1;
                    self.num = self.num.clone() / p.clone();
                }
                return Some(Factor {
                    base: p.clone(),
                    exp,
                });
            }
        }

        unreachable!()
    }
}

/// A factorized number that provides multiply and divide methods which try to avoid overflow.
pub struct Factorized<'a, T> {
    ps: &'a PrimeSeq,
    factors: HashMap<T, i32>,
}

impl<'a, T: Factorize + Eq + Hash> Factorized<'a, T> {
    /// Create a new factorized number representing the integer `1`.
    pub fn new(ps: &PrimeSeq) -> Factorized<'_, T> {
        Factorized {
            ps,
            factors: HashMap::new(),
        }
    }

    /// Create a factorized number from an integer type.
    pub fn from_integer(ps: &PrimeSeq, n: T) -> Factorized<'_, T> {
        Factorized {
            ps,
            factors: n.factorize(ps).map(|f| (f.base, f.exp)).collect(),
        }
    }

    /// Convert the factorized number into an integer type.
    pub fn into_integer(self) -> T {
        self.factors
            .into_iter()
            .fold::<T, _>(One::one(), |prod, (base, exp)| {
                if exp >= 0 {
                    prod * num_traits::pow(base, exp as usize)
                } else {
                    prod / num_traits::pow(base, (-exp) as usize)
                }
            })
    }

    /// Multiplies the given number into the factorized number.
    pub fn mul(&mut self, n: T) {
        for factor in n.factorize(self.ps) {
            match self.factors.entry(factor.base) {
                Vacant(entry) => {
                    entry.insert(factor.exp);
                }
                Occupied(entry) => {
                    *entry.into_mut() += factor.exp;
                }
            }
        }
    }

    /// Divides the factorized number by the given number.
    pub fn div(&mut self, n: T) {
        for factor in n.factorize(self.ps) {
            match self.factors.entry(factor.base) {
                Vacant(entry) => {
                    entry.insert(-factor.exp);
                }
                Occupied(entry) => {
                    *entry.into_mut() -= factor.exp;
                }
            }
        }
    }
}

/// A struct used to cache computed primes.
struct PrimeInner {
    computed: Vec<u64>,
}

impl PrimeInner {
    #[inline]
    fn new() -> Self {
        let mut computed = Vec::with_capacity(INITIAL_CAPACITY);
        computed.push(SEED_PRIMES[0]);
        computed.push(SEED_PRIMES[1]);
        PrimeInner { computed }
    }

    #[inline]
    fn seeded(capacity: usize) -> Self {
        let mut computed = Vec::with_capacity(capacity + SEED_PRIMES.len());
        computed.extend(SEED_PRIMES.iter());
        PrimeInner { computed }
    }

    #[inline]
    fn max_computed(&self) -> u64 {
        *self.computed.last().unwrap()
    }

    #[inline]
    fn compute_nth(&mut self, n: usize) -> u64 {
        self.grow_to_len(n + 1);
        self.computed[n]
    }

    #[inline]
    fn check_prime(&mut self, n: u64) -> bool {
        if n <= self.max_computed() {
            return self.computed.binary_search(&n).is_ok();
        }

        if !self.is_coprime_to_all_computed(n) {
            return false;
        }

        (self.computed.len()..)
            .map(|i| self.compute_nth(i))
            .take_while(|&p| p * p <= n)
            .all(|p| !n.is_multiple_of(&p))
    }

    #[inline]
    fn is_coprime_to_all_computed(&self, n: u64) -> bool {
        self.computed
            .iter()
            .take_while(|&&p| p * p <= n)
            .all(|&p| !n.is_multiple_of(&p))
    }

    #[inline]
    fn grow_to_len(&mut self, len: usize) {
        if self.computed.len() >= len {
            return;
        }

        for n in (self.max_computed() + 2..).step_by(2) {
            if self.is_coprime_to_all_computed(n) {
                self.computed.push(n);

                if self.computed.len() >= len {
                    return;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Factorize, PrimeSeq};

    #[test]
    fn iter() {
        assert_eq!(
            super::SEED_PRIMES,
            &PrimeSeq::new()
                .iter()
                .take(super::SEED_PRIMES.len())
                .collect::<Vec<_>>()[..]
        )
    }

    #[test]
    fn is_prime() {
        let primes = PrimeSeq::new();
        assert!(!primes.is_prime(0));
        assert!(!primes.is_prime(1));
        assert!(primes.is_prime(2));
        assert!(primes.is_prime(3));
        assert!(primes.is_prime(5));
        assert!(primes.is_prime(7));
        assert!(primes.is_prime(11));
        assert!(primes.is_prime(13));
        assert!(!primes.is_prime(99));
        assert!(!primes.is_prime(100));
    }

    #[test]
    fn num_divisor() {
        let pairs = &[
            (0, 0),
            (1, 1),
            (2, 2),
            (3, 2),
            (4, 3),
            (5, 2),
            (12, 6),
            (24, 8),
            (36, 9),
            (48, 10),
            (60, 12),
            (50, 6),
        ];

        let ps = PrimeSeq::new();
        for &(n, num_div) in pairs {
            assert_eq!(num_div, n.num_divisors(&ps));
            assert_eq!(num_div, (-n).num_divisors(&ps));
        }
    }
}
