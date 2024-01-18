use utils::integers::Integer;

fn main() {
    println!("Problem 4: Largest Palindrome Product => {0}", largest_palindrome_product(3));
}

const RADIX: usize = 10;

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
    return n;
}

#[test]
fn validate() {
    assert_eq!(largest_palindrome_product(3), 906609);
}
