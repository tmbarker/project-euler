#[macro_export]
macro_rules! register_problem {
    ($name:expr, $solve_fn:expr, $answer:expr) => {
        #[cfg(not(test))]
        fn main() {
            let solver = $crate::solver::Solver::FunctionOnly($solve_fn);
            let result = solver.run();
            println!("[{}][{:?}] => {}", $name, result.duration, result.answer);
        }

        #[cfg(test)]
        #[test]
        fn validate() {
            let solver = $crate::solver::Solver::FunctionOnly($solve_fn);
            let result = solver.run();
            assert_eq!(result.answer, $answer);
        }
    };

    ($name:expr, $input_file:expr, $solve_fn:expr, $answer:expr) => {
        #[cfg(not(test))]
        fn main() {
            let solver = $crate::solver::Solver::FunctionWithFile($solve_fn, $input_file.to_string());
            let result = solver.run();
            println!("[{}][{:?}] => {}", $name, result.duration, result.answer);
        }

        #[cfg(test)]
        #[test]
        fn validate() {
            let solver = $crate::solver::Solver::FunctionWithFile($solve_fn, $input_file.to_string());
            let result = solver.run();
            assert_eq!(result.answer, $answer);
        }
    };
}