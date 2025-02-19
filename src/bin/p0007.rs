fn solve() -> String {
    nth_prime(10001).to_string()
}

fn nth_prime(n: usize) -> u64 {
    prime::PrimeSeq::seeded(n).nth(n - 1)
}

euler::register_problem!("10001st Prime", solve, "104743");
