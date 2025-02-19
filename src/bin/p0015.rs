fn solve() -> String {
    count_paths(20).to_string()
}

fn count_paths(size: u64) -> u64 {
    prime::PrimeSeq::new().combinations(2 * size, size)
}

euler::register_problem!("Lattice Paths", solve, "137846528820");
