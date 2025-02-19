use std::time::{Duration, Instant};
use std::fs;
use std::path::PathBuf;

const CACHE_DIR: &str = "cache";

pub struct SolverResult {
    pub answer: String,
    pub duration: Duration,
}

pub enum Solver {
    FunctionOnly(fn() -> String),
    FunctionWithFile(fn(&str) -> String, String),
}

impl Solver {
    pub fn run(&self) -> SolverResult {
        let start = Instant::now();
        let answer = match self {
            Solver::FunctionOnly(func) => func(),
            Solver::FunctionWithFile(func, file_name) => {
                let fp = Self::get_cached_file_path(file_name);
                let fc = fs::read_to_string(&fp)
                    .unwrap_or_else(|_| panic!("Failed to read input file: {:?}", fp));
                func(&fc)
            },
        };

        SolverResult
        {
            answer,
            duration: start.elapsed()
        }
    }

    fn get_cached_file_path(file_name: &str) -> PathBuf {
        PathBuf::from(CACHE_DIR).join(file_name)
    }
}