use integer::Integer;

fn solve() -> String {
    (1..1_000_000)
        .filter(|&n| n.is_palindromic(10) && n.is_palindromic(2))
        .sum::<usize>()
        .to_string()
}

euler::register_problem!("Double-base Palindromes", solve, "872187");