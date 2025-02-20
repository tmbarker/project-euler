use prime::PrimeSeq;

fn solve() -> String {
    //  NOTE: n^2 + an + b reduces to simply b when n is zero. Therefore, we only need to consider
    //  a-b pairs where b is prime.
    //
    let ps = PrimeSeq::new();
    let cs = ps
        .iter()
        .take_while(|&p| p <= 1000)
        .flat_map(|b| (-999..1000).map(move |a| (a, b as i64)))
        .max_by_key(|&(a, b)| count_primes(&a, &b, &ps))
        .unwrap();

    let (a, b) = cs;
    let p = a * b;

    p.to_string()
}

fn count_primes(a: &i64, b: &i64, p: &PrimeSeq) -> i64 {
    (0..)
        .take_while(|x| {
            let y = x * x + a * x + b;
            y > 0 && p.contains(y as u64)
        })
        .count() as i64
}

euler::register_problem!("Quadratic Primes", solve, "-59231");