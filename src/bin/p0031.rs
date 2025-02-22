fn solve() -> String {
    const TARGET: usize = 200;
    let coins = vec![1, 2, 5, 10, 20, 50, 100, 200];
    let mut ways = vec![0; TARGET + 1];
    ways[0] = 1;

    for coin in coins {
        for amount in coin..=TARGET {
            ways[amount] += ways[amount - coin];
        }
    }

    ways[TARGET].to_string()
}

euler::register_problem!("Coin Sums", solve, "73682");