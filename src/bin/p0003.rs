use prime::{Factorize, PrimeSeq};

fn solve() -> String {
    largest_prime_factor(600851475143).to_string()
}

fn largest_prime_factor(x: u64) -> u64 {
    x.factorize(&PrimeSeq::new())
        .map(|factor| factor.base)
        .max()
        .unwrap()
}

euler::register_problem!("Largest Prime Factor", solve, "6857");