use num_bigint::BigUint;
use num_traits::one;
use utils::seqs::FibonacciIter;

fn main() {
    println!(
        "Problem 25: 1000-digit Fibonacci Number => {0}",
        n_digit_fibonacci(1000)
    );
}

fn n_digit_fibonacci(d: u32) -> usize {
    let max = BigUint::from(10u32).pow(d - 1) - one::<BigUint>();
    FibonacciIter::<BigUint>::start_at(one::<BigUint>(), one::<BigUint>())
        .take_while(|n| *n <= max)
        .count()
        + 1
}

#[test]
fn validate() {
    assert_eq!(n_digit_fibonacci(1000), 4782);
}
