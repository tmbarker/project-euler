use num_integer::Integer;
use utils::seqs::FibonacciIter;

fn main() {
    println!(
        "Problem 2: Even Fibonacci Numbers => {0}",
        even_fibonacci(4000000)
    );
}

fn even_fibonacci(x: u32) -> u32 {
    FibonacciIter::<u32>::new()
        .take_while(|&n| n <= x)
        .filter(|&n| n.is_even())
        .sum()
}

#[test]
fn validate() {
    assert_eq!(even_fibonacci(4000000), 4613732);
}
