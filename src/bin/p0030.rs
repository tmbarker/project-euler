use integer::Integer;

fn solve() -> String {
    //  NOTE: The upper bound of the search space can be found by computing the maximum contribution
    //  from a single digit, and then figuring out how many digits can be summed before they exceed
    //  the maximum magnitude of a number with that many digits:
    //      9^5 = 59049
    //      n * 59049 <= 10^n
    //      n <= 6
    //
    (2..6 * 59049)
        .filter(|&n| check(n))
        .sum::<usize>()
        .to_string()
}

fn check(n: usize) -> bool {
    n == n
        .to_digits(10)
        .map(|d| d.pow(5))
        .sum()
}

euler::register_problem!("Digit Fifth Powers", solve, "443839");