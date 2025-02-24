use integer::Integer;
use prime::PrimeSeq;

fn solve() -> String {
    let primes = PrimeSeq::new();
    primes
        .iter()
        .take_while(|&p| p < 1_000_000)
        .filter(|&p| is_circular(&primes, p))
        .count()
        .to_string()
}

fn is_circular(ps: &PrimeSeq, p: u64) -> bool {
    const RADIX: u64 = 10;
    let digits = p.into_digits(RADIX).collect::<Vec<_>>();
    let length = digits.len();

    (1..length)
        .map(|i| u64::from_digits(digits
            .iter()
            .copied()
            .cycle()
            .skip(i)
            .take(length), RADIX))
        .all(|r| ps.contains(r))
}

euler::register_problem!("Circular Primes", solve, "55");
