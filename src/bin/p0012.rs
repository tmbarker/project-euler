use utils::primes::{Factorize, PrimeSeq};
use utils::seqs::TriangularIter;

fn main() {
    println!(
        "Problem 12: Highly Divisible Triangular Number => {0}",
        triangular_min_divisors(500)
    );
}

fn triangular_min_divisors(num_div: u64) -> u64 {
    let ps = PrimeSeq::new();
    TriangularIter::<u64>::new()
        .find(|&n| n.num_divisors(&ps) > num_div)
        .unwrap()
}

#[test]
fn validate() {
    assert_eq!(triangular_min_divisors(500), 76576500)
}
