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

| *Day* | *Part 1* | *Part 2* |
|-------|----------|----------|
|   1   |   2.4 µs | 121.6 µs |
|   2   |  12.2 µs |  15.8 µs |
|   3   |   5.3 µs |  25.1 µs |
|   4   |   1.5 µs |   1.8 µs |
|   5   |  15.3 µs |  17.9 µs |
|   6   |   140 ns |  14.2 ms |
