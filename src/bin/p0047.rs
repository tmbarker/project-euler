use prime::{Factorize, PrimeSeq};

fn solve() -> String {
    let ps = PrimeSeq::new();
    (1..)
        .find(|&n| {
            (n..n + 4)
                .map(|n| n.factorize(&ps).count())
                .all(|num_factors| num_factors == 4)
        })
        .unwrap()
        .to_string()
}

euler::register_problem!("Distinct Primes Factors", solve, "134043");