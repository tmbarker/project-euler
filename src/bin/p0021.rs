use prime::{Factorize, PrimeSeq};

fn solve() -> String {
    sum_divisors(10000).to_string()
}

fn sum_divisors(n: usize) -> usize {
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

euler::register_problem!("Amicable Numbers", solve, "31626");
