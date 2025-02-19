#[macro_export]
macro_rules! register_problem {
    ($name:expr, $solve_fn:expr, $answer:expr) => {
        #[cfg(not(test))]
        fn main() {
            let result = $solve_fn();
            println!("{} => {}", $name, result);
        }

        #[cfg(test)]
        #[test]
        fn validate() {
            let result = $solve_fn();
            assert_eq!(result, $answer);
        }
    };
}