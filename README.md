# aoc-2023
My advent of code solutions for 2023

# Building
A nightly version of rust is required, for debug builds:
```
cargo run -- -d <DAY>
```
If you want to run benchmarks:
```
cargo build --release
target/release/aoc-2023 -d <DAY> --bench
```

# Input
Inputs are stored in the `inputs` folder in the working directory when running the binary.
They are stored as `<DAY>.txt` and can be automatically downloaded if you set the environment variable AOC_SESSION to your
session token. A .env file can be used to do this automatically.

# Benchmarks
On a recent linux version and a Ryzen 5950X with 3600 MT/s DDR4 RAM.
Times include "parsing", i.e. the benchmark measures from the moment the input file is in RAM.

-------------------------------
| *Day* | *Part 1* | *Part 2* |
|------------------------------
|   1   |    3 µs  |    -     |
|   2   |   13 µs  |   15 µs  |
------------------------------
