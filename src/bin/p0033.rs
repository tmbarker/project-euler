use num_integer::Integer;

fn solve() -> String {
    let (pn, pd) =(10..100)
        .flat_map(|n| (n + 1..=100).map(move |d| (n, d)))
        .filter(|&(n, d)| n % 10 != 0 || d % 10 != 0)
        .filter(|&(n, d)| {
            let (n1, n2) = (n / 10, n % 10);
            let (d1, d2) = (d / 10, d % 10);
            (n1 == d2 && n2 * d == n * d1) || (n2 == d1 && n1 * d == n * d2)
        })
        .fold((1, 1), |(np, nd), (n, d)| (n * np, d * nd));

    let gcd = pn.gcd(&pd);
    let den = pd / gcd;

    den.to_string()
}

euler::register_problem!("Digit Cancelling Fractions", solve, "100");