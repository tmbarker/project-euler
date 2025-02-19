fn solve() -> String {
    sum_square_difference(100).to_string()
}

fn sum_square_difference(n: usize) -> usize {
    let sum_of_square = n * (n + 1) * (2 * n + 1) / 6;
    let sum_of_values = n * (n + 1) / 2;

    sum_of_values * sum_of_values - sum_of_square
}

euler::register_problem!("Sum Square Difference", solve, "25164150");
