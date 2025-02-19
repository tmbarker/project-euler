fn solve() -> String {
    smallest_multiple(20).to_string()
}

fn smallest_multiple(x: usize) -> usize {
    (1..=x).fold(1, num_integer::lcm)
}

euler::register_problem!("Smallest Multiple", solve, "232792560");
