fn main() {
    println!("Problem 16: Power Digit Sum => {0}", pow2_digit_sum(1000));
}

fn pow2_digit_sum(n: u64) -> u64 {
    (0..n)
        .fold(vec![1], |digits, _| double(digits))
        .iter()
        .sum()
}

fn double(mut digits: Vec<u64>) -> Vec<u64> {
    const RADIX: u64 = 10;
    let mut carry = 0;

    for i in 0..digits.len() {
        digits[i] += digits[i] + carry;
        carry = digits[i] / RADIX;
        digits[i] %= RADIX;
    }

    while carry > 0 {
        digits.push(carry % RADIX);
        carry /= RADIX;
    }

    digits
}

#[test]
fn validate() {
    assert_eq!(pow2_digit_sum(0), 1);
    assert_eq!(pow2_digit_sum(1), 2);
    assert_eq!(pow2_digit_sum(2), 4);
    assert_eq!(pow2_digit_sum(4), 7);
    assert_eq!(pow2_digit_sum(1000), 1366);
}
