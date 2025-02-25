use integer::Integer;
use prime::PrimeSeq;

fn solve() -> String {
    let primes = PrimeSeq::new();
    primes
        .iter()
        .skip_while(|&n| n <= 7)
        .filter(|&n| is_truncatable(&primes, n))
        .take(11)
        .sum::<u64>()
        .to_string()
}

fn is_truncatable(ps: &PrimeSeq, n: u64) -> bool {
    const RADIX: u64 = 10;
    let digits = n.into_digits(RADIX).collect::<Vec<_>>();
    let length = digits.len();

    (1..length)
        .map(|i| {
            let a = u64::from_digits(digits.iter().copied().skip(i), RADIX);
            let b = u64::from_digits(digits.iter().copied().take(length - i), RADIX);
            (a, b)
        })
        .all(|(a, b)| ps.contains(a) && ps.contains(b))
}

euler::register_problem!("Truncatable Primes", solve, "748317");