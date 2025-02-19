use integer::Integer;

const RADIX: usize = 10;

fn solve() -> String {
    largest_palindrome_product(3).to_string()
}

fn largest_palindrome_product(d: u32) -> usize {
    let min = RADIX.pow(d - 1);
    let max = RADIX.pow(d) - 1;

    let mut p: usize;
    let mut n: usize = 0;

    for a in (min..=max).rev() {
        for b in (a..=max).rev() {
            p = a * b;
            if p <= n {
                break;
            }
            if p.is_palindromic(RADIX) {
                n = std::cmp::max(n, p);
            }
        }
    }
    n
}

euler::register_problem!("Largest Palindrome Product", solve, "906609");
