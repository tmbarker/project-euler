use num_bigint::BigUint;

fn solve() -> String {
    factorial_digit_sum(100).to_string()
}

fn factorial_digit_sum(x: u32) -> u32 {
    (1..=x)
        .map(BigUint::from)
        .fold(num_traits::one::<BigUint>(), |acc, n| acc * n)
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .sum()
}

euler::register_problem!("Factorial Digit Sum", solve, "648");
