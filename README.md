# Project Euler

## Introduction
[Project Euler](https://projecteuler.net/about) is a free series of mathematical problems, most of which are related to number theory. I decided to use Project Euler as an opportunity to learn Rust; as such I expect that my solutions will generally improve in quality and become more idiomatic as I progress through the project.

## Running a Solution
Each problem solution is implemented in its own binary, and you can run any solution from the CLI as follows:
```bash
# The solution number should be prepended with the letter 'p', and padded to a length of 4
cargo run --bin p<number:0000>

# E.g.:
cargo run --bin p0001
cargo run --bin p0002
```
