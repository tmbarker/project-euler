use std::collections::HashMap;

fn solve() -> String {
    (2..1000)
        .max_by_key(|&n| get_reciprocal_cycle(n))
        .unwrap()
        .to_string()
}

fn get_reciprocal_cycle(n: usize) -> usize {
    let mut seen: HashMap<usize, usize> = HashMap::new();
    let mut rem = 1;
    let mut pos = 0;

    loop {
        rem = rem % n;

        if rem == 0 {
            return 0;
        }
        if let Some(&prev) = seen.get(&rem) {
            return pos - prev;
        }

        seen.insert(rem, pos);
        rem *= 10;
        pos += 1;
    }
}

euler::register_problem!("Reciprocal Cycles", solve, "983");