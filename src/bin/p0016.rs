fn solve() -> String {
    pow2_digit_sum(1000).to_string()
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
        digits[i] = 2 * digits[i] + carry;
        carry = digits[i] / RADIX;
        digits[i] %= RADIX;
    }

    while carry > 0 {
        digits.push(carry % RADIX);
        carry /= RADIX;
    }

    digits
}

euler::register_problem!("Power Digit Sum", solve, "1366");
