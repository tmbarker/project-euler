fn main() {
    println!("Problem 1: Multiples of 3 or 5 => {0}", sum_multiples(1000));
}

fn sum_multiples(x: u32) -> u32 {
    (1..x).filter(|&n| n % 3 == 0 || n % 5 == 0).sum()
}

#[test]
fn validate() {
    assert_eq!(sum_multiples(10), 23);
    assert_eq!(sum_multiples(1000), 233168)
}
