use integer::Integer;

fn solve() -> String {
    let targets = [10, 100, 1_000, 10_000, 100_000, 1_000_000];
    (1..)
        .flat_map(|n| n.into_digits(10).rev())
        .enumerate()
        .filter(|&(i, _)| targets.contains(&(i + 1)))
        .take(targets.len())
        .fold(1, |acc, (_, d)| acc * d)
        .to_string()
}

euler::register_problem!("Champernowne's Constant", solve, "210");
