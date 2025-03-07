use integer::Integer;
use iter::CombinationIter;
use prime::PrimeSeq;

const RADIX: u64 = 10;
const COUNT: usize = 8;

fn solve() -> String {
    let primes = PrimeSeq::new();
    let mut family = Vec::with_capacity(COUNT);
    primes
        .iter()
        .skip_while(|&p| p < RADIX)
        .filter_map(|p| find_family(&primes, p, &mut family))
        .next()
        .unwrap()
        .to_string()
}

fn find_family(primes: &PrimeSeq, seed: u64, family: &mut Vec<u64>) -> Option<u64> {
    let p_digits = seed.into_digits(RADIX).collect::<Vec<_>>();
    let p_length = p_digits.len();
    let p_indices = (0..p_length).collect::<Vec<_>>();

    for r_count in 1..p_length {
        for r_indices in CombinationIter::new(&p_indices, r_count) {
            family.clear();
            let r_min = if r_indices[r_count - 1] == (p_length - 1) { 1 } else { 0 };
            for r in r_min..RADIX {
                let pc = construct(&p_digits, &r_indices, r);
                if primes.contains(pc) {
                    family.push(pc);
                }

                if family.len() >= COUNT {
                    return Some(family[0]);
                }
            }
        }
    }

    None
}

fn construct(p_digits: &[u64], r_indices: &[usize], r: u64) -> u64 {
    let mut order = 1;
    let mut result = 0;

    for i in 0..p_digits.len() {
        if r_indices.contains(&i) {
            result += order * r;
        } else {
            result += order * p_digits[i];
        }
        order *= RADIX;
    }

    result
}

euler::register_problem!("Prime Digit Replacements", solve, "121313");
