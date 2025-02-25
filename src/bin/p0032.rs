use integer::Integer;
use iter::PermutationIter;
use std::collections::HashSet;

const DIGITS: usize = 9;
const RADIX: usize = 10;

fn solve() -> String {
    let digits = (1..=DIGITS).collect::<Vec<usize>>();
    let mut perm = PermutationIter::new(&digits, DIGITS);
    let mut prod = HashSet::new();

    while let Some((p, _)) = perm.next() {
        check(p, &mut prod);
    }

    prod
        .iter()
        .sum::<usize>()
        .to_string()
}

fn check(digits: Vec<usize>, products: &mut HashSet<usize>) {
    for i in 1..=(DIGITS / 2) {
        for j in (i + 1)..=(i + 2) {
            let m = usize::from_rev_digits(digits.iter().take(i).copied(), RADIX);
            let n = usize::from_rev_digits(digits.iter().skip(i).take(j - i).copied(), RADIX);
            let p = usize::from_rev_digits(digits.iter().skip(j).copied(), RADIX);

            if m * n == p {
                products.insert(p);
            }
        }
    }
}

euler::register_problem!("Pandigital Products", solve, "45228");