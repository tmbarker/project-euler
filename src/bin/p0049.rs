use integer::Integer;
use iter::CombinationIter;
use prime::PrimeSeq;
use std::collections::HashMap;

const PRIMES: [u64; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
const RADIX: u64 = 10;
const KNOWN: u64 = 1487;

fn solve() -> String {
    let prime_sets: Vec<Vec<u64>>  = PrimeSeq::new()
        .iter()
        .skip_while(|&p| p < 01_000)
        .take_while(|&p| p < 10_000)
        .fold(HashMap::<u64, Vec<u64>>::new(), |mut map, p| {
            map.entry(hash(p)).or_default().push(p);
            map
        })
        .into_values()
        .filter(|g| g.len() >= 3)
        .collect();

    for group in prime_sets{
        for comb in CombinationIter::new(&group, 3){
            let mut seq = comb;
            seq.sort();
            let a = seq[0];
            let b = seq[1];
            let c = seq[2];

            if c - b == b - a && ![a, b, c].contains(&KNOWN) {
                return format!("{}{}{}", a, b, c);
            }
        }
    }

    unreachable!("No solution found");
}

fn hash(prime: u64) -> u64 {
    prime
        .into_digits(RADIX)
        .map(|d| PRIMES[d as usize])
        .product()
}

euler::register_problem!("Prime Permutations", solve, "296962999629");