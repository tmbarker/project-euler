use crate::utilities::sequences;

pub fn solve() {
    println!("Problem 2: Even Fibonacci Numbers => {0}", even_fibonacci(4000000));
}

fn even_fibonacci(x: u32) -> u32 {
    sequences::Fibonacci::<u32>::new()
        .take_while(|&n| n <= x)
        .filter(|&n| n % 2 == 0)
        .sum()
}

#[test]
fn validate() {
    assert_eq!(even_fibonacci(4000000), 4613732);
}
