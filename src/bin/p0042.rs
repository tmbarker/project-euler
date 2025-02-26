use std::collections::HashSet;
use seq::TriangularIter;

fn solve(input: &str) -> String {
    let words = input
        .split(',')
        .map(|name| name.trim_matches('"'))
        .map(|name| name
            .to_ascii_uppercase()
            .bytes()
            .map(|c| (c - b'A' + 1) as usize)
            .sum::<usize>())
        .collect::<Vec<_>>();

    let max = words.iter().max().unwrap();
    let tri = TriangularIter::new()
        .take_while(|&t| t <= *max)
        .collect::<HashSet<usize>>();

    words
        .iter()
        .filter(|&word| tri.contains(word))
        .count()
        .to_string()
}

euler::register_problem!("Coded Triangle Numbers", "0042_words.txt", solve, "162");