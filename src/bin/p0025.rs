use num_bigint::BigUint;
use num_traits::one;
use seq::FibonacciIter;

fn solve() -> String {
    n_digit_fibonacci(1000).to_string()
}

fn n_digit_fibonacci(d: u32) -> usize {
    let max = BigUint::from(10u32).pow(d - 1) - one::<BigUint>();
    FibonacciIter::<BigUint>::start_at(one::<BigUint>(), one::<BigUint>())
        .take_while(|n| *n <= max)
        .count()
        + 1
}

euler::register_problem!("1000-digit Fibonacci Number", solve, "4782");
