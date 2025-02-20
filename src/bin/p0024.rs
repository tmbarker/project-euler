fn solve() -> String {
    (1..1_000_000)
        .fold([0, 1, 2, 3, 4, 5, 6, 7, 8, 9], |mut acc, _| {
            next_permutation(&mut acc);
            acc
        })
        .iter()
        .map(|d| d.to_string())
        .collect::<String>()
}

fn next_permutation(a: &mut [usize]){
    let mut k = a.len() - 2;
    while a[k] >= a[k + 1] {
        k -= 1;
    }

    let mut l = a.len() - 1;
    while a[k] >= a[l] {
        l -= 1;
    }

    a.swap(k, l);
    a[k + 1..].reverse();
}

euler::register_problem!("Lexicographic Permutations", solve, "2783915460");