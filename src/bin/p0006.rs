fn main() {
    println!("Problem 6: Sum Square Difference => {0}", sum_square_difference(100));
}

fn sum_square_difference(n: usize) -> usize {
    let sum_of_square = n * (n + 1) * (2 * n + 1) / 6;
    let sum_of_values = n * (n + 1) / 2;

    sum_of_values * sum_of_values - sum_of_square
}

#[test]
fn validate() {
    assert_eq!(sum_square_difference(100), 25164150)
}