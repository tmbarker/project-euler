use std::collections::HashSet;
use prime::{PrimeSeq, Factorize, Factor};

fn solve() -> String {
    count_distinct_powers(2, 100).to_string()
}

fn count_distinct_powers(min: i32, max: i32) -> usize {
    let mut set = HashSet::new();
    let ps = PrimeSeq::new();

    for a in min..=max {
        let a_factors = a.factorize(&ps).collect::<Vec<_>>();
        for b in min..=max {
            set.insert(a_factors
                .iter()
                .map(|a_factor| Factor {base: a_factor.base, exp: a_factor.exp * b})
                .collect::<Vec<_>>());
        }
    }

    set.len()
}

euler::register_problem!("Distinct Powers", solve, "9183");