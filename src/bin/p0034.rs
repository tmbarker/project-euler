use integer::Integer;

fn solve() -> String {
    let fs = (0..10)
        .map(|n| n.factorial())
        .collect::<Vec<usize>>();

    //  NOTE: We can find an upper bound for the search space by comparing the growth rate of
    //  factorial digit sums vs the numbers themselves. The maximum factorial digit is 9!, and
    //  the smallest number with n digits is 10^(n-1). We can find the smallest n such that
    //  n * 9! >= 10^(n-1): n = 7. Therefore, the maximum number we need to check is 7 * 9!.
    //
    //  However, while it is guaranteed that no factorions exist above 7 * 9!, in practice only
    //  4 exist (1, 2, 145, and 40585). Therefor in practice we can use a much lower upper bound.
    //
    (10..50_000)
        .filter(|&n| {
            n == n.to_digits(10)
                .map(|d| fs[d])
                .sum()
        })
        .sum::<usize>()
        .to_string()
}

euler::register_problem!("Digit Factorials", solve, "40730");