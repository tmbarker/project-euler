use utils::primes::{PrimeGen, Factorize};

fn main(){
    println!("Problem 3: Largest Prime Factor => {0}", largest_prime_factor(600851475143));
}

fn largest_prime_factor(x: u64) -> u64 {
    x.factorize(&PrimeGen::new())
        .map(|factor| factor.base)
        .max()
        .unwrap()
}

#[test]
fn validate() {
    assert_eq!(largest_prime_factor(100), 5);
    assert_eq!(largest_prime_factor(600851475143), 6857);
}