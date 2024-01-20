use std::collections::HashMap;
use utils::seqs::CollatzIter;

fn main() {
    println!(
        "Problem 14: Longest Collatz Sequence => {0}",
        longest_collatz(1000000)
    );
}

fn longest_collatz(bound: usize) -> usize {
    let mut memo = HashMap::with_capacity(bound);
    let _ = memo.insert(1, 1);

    (2..bound)
        .max_by_key(|&n| get_length(&mut memo, n))
        .unwrap()
}

#[inline]
fn get_length(memo: &mut HashMap<usize, usize>, start: usize) -> usize {
    for (i, n) in CollatzIter::start_at(start).enumerate() {
        if let Some(&x) = memo.get(&n) {
            memo.insert(start, x + i);
            return x + i;
        }
    }
    unreachable!()
}

#[test]
fn validate() {
    assert_eq!(longest_collatz(1000000), 837799)
}
