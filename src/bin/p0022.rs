fn solve(input: &str) -> String {
    let mut words = input
        .split(',')
        .map(|name| name.trim_matches('"'))
        .collect::<Vec<_>>();

    words.sort();
    words
        .iter()
        .enumerate()
        .map(|(i, word)| score(i + 1, word))
        .sum::<usize>()
        .to_string()
}

fn score(n: usize, name: &str) -> usize {
    n * name
        .bytes()
        .map(|c| (c - b'A' + 1) as usize)
        .sum::<usize>()
}

euler::register_problem!("Names Scores", "0022_names.txt", solve, "871198282");