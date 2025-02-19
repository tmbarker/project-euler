fn solve() -> String {
    sum_multiples(1000).to_string()
}

fn sum_multiples(x: u32) -> u32 {
    (1..x).filter(|&n| n % 3 == 0 || n % 5 == 0).sum()
}

euler::register_problem!("Multiples of 3 or 5", solve, "233168");