fn main() {
    println!("Problem 5: Smallest Multiple => {0}", smallest_multiple(20));
}

fn smallest_multiple(x: usize) -> usize {
    (1..=x).fold(1, |lcm, n| num_integer::lcm(lcm, n))
}

#[test]
fn validate() {
    assert_eq!(smallest_multiple(20), 232792560);
}
