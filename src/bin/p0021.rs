use utils::primes::{Factorize, PrimeSeq};

fn main() {
    println!(
        "Problem 21: Amicable Numbers => {0}",
        solve(10000)
    );
}

fn solve(n: usize) -> usize {
    let primes_seq = PrimeSeq::new();
    let sum_of_divs = (0..=n)
        .map(|n| n.sum_proper_divisors(&primes_seq))
        .collect::<Vec<_>>();

    sum_of_divs
        .iter()
        .enumerate()
        .filter(|&(n, &sum)| sum < n && sum_of_divs[sum] == n)
        .map(|(a, b)| a + b)
        .sum()
}

#[test]
fn validate() {
    assert_eq!(solve(10000), 31626);
}