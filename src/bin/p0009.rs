use num_integer::Roots;
use std::{cmp, iter};

fn solve() -> String {
    find_triplet_product(1000).to_string()
}

fn find_triplet_product(sum: u64) -> u64 {
    (3..sum - 1)
        .flat_map(|c| {
            let a_max = cmp::min((sum - c) / 2, (c * c / 2).sqrt());
            (1..a_max).zip(iter::repeat(c))
        })
        .map(|(a, c)| (a, sum - a - c, c))
        .find(|&(a, b, c)| a * a + b * b == c * c)
        .map(|(a, b, c)| a * b * c)
        .unwrap()
}

euler::register_problem!("Special Pythagorean Triplet", solve, "31875000");
