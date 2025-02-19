use prime::{Factorize, PrimeSeq};

const LIMIT: usize = 28123;

fn solve() -> String {
    let primes = PrimeSeq::new();
    let abundant = (12..=LIMIT)
        .filter(|&n| n.sum_proper_divisors(&primes) > n)
        .collect::<Vec<_>>();

    let mut sum_of_abundant_composites = 0;
    let mut is_abundant_sum = vec![false; LIMIT + 1];

    for (i, &a) in abundant.iter().enumerate() {
        for &b in &abundant[i..]{
            let s = a + b;
            if s > LIMIT {
                break
            }
            if !is_abundant_sum[s] {
                sum_of_abundant_composites += s;
                is_abundant_sum[s] = true;
            }
        }
    }

    let sum_of_all = LIMIT * (LIMIT + 1) / 2;
    let sum_of_non_composite = sum_of_all - sum_of_abundant_composites;
    sum_of_non_composite.to_string()
}

euler::register_problem!("Non-Abundant Sums", solve, "4179871");