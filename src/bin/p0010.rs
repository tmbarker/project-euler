fn main() {
    println!(
        "Problem 10: Summation of Primes => {0}",
        sum_primes(2000000)
    );
}

fn sum_primes(n: u64) -> u64 {
    utils::primes::PrimeSeq::new()
        .iter()
        .take_while(|&p| p < n)
        .sum()
}

#[test]
fn validate() {
    assert_eq!(sum_primes(2000000), 142913828922)
}
