use num_integer::Integer;
use seq::FibonacciIter;

fn solve() -> String {
    even_fibonacci(4000000).to_string()
}

fn even_fibonacci(x: u32) -> u32 {
    FibonacciIter::<u32>::new()
        .take_while(|&n| n <= x)
        .filter(|&n| n.is_even())
        .sum()
}

euler::register_problem!("Even Fibonacci Numbers", solve, "4613732");