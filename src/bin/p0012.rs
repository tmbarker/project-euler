use prime::{Factorize, PrimeSeq};
use seq::TriangularIter;

fn solve() -> String {
    triangular_min_divisors(500).to_string()
}

fn triangular_min_divisors(num_div: u64) -> u64 {
    let ps = PrimeSeq::new();
    TriangularIter::<u64>::new()
        .find(|&n| n.num_divisors(&ps) > num_div)
        .unwrap()
}

euler::register_problem!("Highly Divisible Triangular Number", solve, "76576500");
