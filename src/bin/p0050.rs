use prime::PrimeSeq;

const LIMIT: u64 = 1_000_000;

fn solve() -> String {
    let prime_seq = PrimeSeq::new();
    let prime_set = prime_seq
        .iter()
        .take_while(|&p| p < LIMIT)
        .collect::<Vec<u64>>();

    let mut sums = vec![0; prime_set.len() + 1];
    for i in 0..prime_set.len() {
        sums[i + 1] = sums[i] + prime_set[i];
    }

    let mut longest = 0;
    let mut result = 0;

    for l in 0..sums.len() {
        for r in (l + longest + 1)..sums.len() {
            let sum = sums[r] - sums[l];
            if sum > LIMIT {
                break;
            }
            if r - l > longest && prime_seq.contains(sum) {
                longest = r - l;
                result = sum;
            }
        }
    }

    result.to_string()
}

euler::register_problem!("Consecutive Prime Sum", solve, "997651");
