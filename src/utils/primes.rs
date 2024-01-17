use num_integer::Integer;
use num_traits::{FromPrimitive, One};
use std::{cell::RefCell, mem, rc::Rc};

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
pub struct PrimeGen {
    data: Rc<RefCell<PrimeInner>>,
}

impl PrimeGen {
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

    /// Compute the nth prime.
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
    pub fn iter(&self) -> PrimeSeq {
        PrimeSeq {
            idx: 0,
            data: self.data.clone(),
        }
    }

    #[inline]
    fn from_inner(gen: PrimeInner) -> Self {
        PrimeGen {
            data: Rc::new(RefCell::new(gen)),
        }
    }
}

/// Prime number sequence iterator.
pub struct PrimeSeq {
    idx: usize,
    data: Rc<RefCell<PrimeInner>>,
}

impl Iterator for PrimeSeq {
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
pub struct Factors<T> {
    num: T,
    primes: PrimeSeq,
}

/// Numbers which can be factorized.
pub trait Factorize: Integer + FromPrimitive + Clone {
    /// An iterator yielding all prime factors in ascending order.
    fn factorize(&self, pg: &PrimeGen) -> Factors<Self>;
}

/// Implement the Factorize trait for an unsigned integer type.
macro_rules! factorize_trait_impl_unsigned {
    ($($t:ty)*) => ($(
        impl Factorize for $t {
            fn factorize(&self, ps: &PrimeGen) -> Factors<$t> {
                Factors { num: *self, primes: ps.iter() }
            }
        }
    )*)
}

/// Implement the Factorize trait for a signed integer type.
macro_rules! factorize_trait_impl_signed {
    ($($t:ty)*) => ($(
        impl Factorize for $t {
            fn factorize(&self, ps: &PrimeGen) -> Factors<$t> {
                if *self < 0 {
                    Factors { num: -*self, primes: ps.iter() }
                } else {
                    Factors { num: *self, primes: ps.iter() }
                }
            }
        }
   )*)
}

factorize_trait_impl_unsigned!(usize u8 u16 u32 u64);
factorize_trait_impl_signed!(isize i8 i16 i32 i64);

impl<T: Integer + FromPrimitive + Clone> Iterator for Factors<T> {
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
