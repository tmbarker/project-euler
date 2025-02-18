use num_bigint::BigUint;

fn main() {
    println!(
        "Problem 20: Factorial Digit Sum => {0}",
        factorial_digit_sum(100)
    );
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

#[test]
fn validate() {
    assert_eq!(factorial_digit_sum(1), 1);
    assert_eq!(factorial_digit_sum(10), 27);
    assert_eq!(factorial_digit_sum(100), 648);
}
