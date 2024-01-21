use utils::primes::PrimeSeq;

fn main() {
    println!("Problem 15: Lattice Paths => {0}", count_paths(20));
}

fn count_paths(size: u64) -> u64 {
    PrimeSeq::new().combinations(2 * size, size)
}

#[test]
fn validate() {
    assert_eq!(count_paths(20), 137846528820);
}
