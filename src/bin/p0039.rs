fn solve() -> String {
    ((3 + 4 + 5)..=1000)
        .max_by_key(|&p| count_ways(p))
        .unwrap()
        .to_string()
}

fn count_ways(p: usize) -> usize {
    let mut ways = 0;
    for a in 1..=(p / 3) {
        for b in (a + 1)..=((p - a) / 2) {
            let c = p - a - b;
            if a * a + b * b == c * c {
                ways += 1;
            }
        }
    }
    ways
}

euler::register_problem!("Integer right triangles", solve, "840");
