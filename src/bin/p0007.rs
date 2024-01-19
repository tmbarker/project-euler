fn main() {
    println!("Problem 7: 10001st Prime => {0}", nth_prime(10001));
}

fn nth_prime(n: usize) -> u64 {
    utils::primes::PrimeSeq::seeded(n).nth(n - 1)
}

#[test]
fn validate() {
    assert_eq!(nth_prime(10001), 104743)
}
