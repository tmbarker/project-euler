use integer::Integer;
use std::collections::HashSet;

const RADIX: usize = 10;
const MULTIPLES: &[usize] = &[2, 3, 4, 5, 6];

fn solve() -> String {
    (1..)
        .find(|&n| check(n))
        .unwrap()
        .to_string()
}

fn check(n: usize) -> bool {
    let a: Vec<_> = (n * MULTIPLES[0]).into_digits(RADIX).collect();
    (1..MULTIPLES.len())
        .all(|i| {
            let b: Vec<_> = (n * MULTIPLES[i]).into_digits(RADIX).collect();
            same_elements(&a, &b)
        })
}

fn same_elements(a: &[usize], b: &[usize]) -> bool {
    let set_a: HashSet<_> = a.iter().collect();
    let set_b: HashSet<_> = b.iter().collect();
    set_a == set_b
}

euler::register_problem!("Permuted Multiples", solve, "142857");