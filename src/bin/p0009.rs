use num_integer::Roots;
use std::{cmp, iter};

fn main() {
    println!(
        "Problem 9: Special Pythagorean Triplet => {0}",
        find_triplet_product(1000)
    );
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

#[test]
fn validate() {
    assert_eq!(find_triplet_product(1000), 31875000)
}
