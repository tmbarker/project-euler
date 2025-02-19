fn solve() -> String {
    sum_primes(2000000).to_string()
}

fn sum_primes(n: u64) -> u64 {
    prime::PrimeSeq::new()
        .iter()
        .take_while(|&p| p < n)
        .sum()
}

euler::register_problem!("Summation of Primes", solve, "142913828922");
