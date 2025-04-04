# fib-rs

[![Crates.io](https://img.shields.io/crates/v/fib-rs)](https://crates.io/crates/fib-rs)
[![Documentation](https://img.shields.io/docsrs/fib-rs)](https://docs.rs/fib-rs)
[![MIT](https://img.shields.io/crates/l/fib-rs)](LICENSE)

A highly optimized Fibonacci number calculator for Rust that efficiently computes arbitrarily large Fibonacci numbers.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Performance](#performance)
- [Algorithm Details](#algorithm-details)
- [License](#license)
- [Contributing](#contributing)

## Features

- **Fast doubling algorithm**: Calculates Fibonacci numbers in O(log n) time
- **Handles massive inputs**: Compute Fibonacci numbers up to F(10,000,000) and beyond
- **Range calculation**: Generate sequences of consecutive Fibonacci numbers with parallel processing
- **CLI application**: Simple command-line interface for quick calculations of single values or ranges

## Installation

### Library

To use as a dependency in your project:

```bash
cargo add fib-rs
```

### CLI Tool

To install the command-line tool:

```bash
cargo install fib-rs
```

## Usage

### As a library

```rust
use fib_rs::{fib, fib_range};

// Calculate F(100)
let i = 100;
let result = fib(i);
// Print the result
println!("F({}) = {}", i, result);

// Calculate a range of Fibonacci numbers (F(3) through F(10))
let results = fib_range(3..=10);
// Print the results
(*start..=*end)
    .zip(results.iter())
    .for_each(|(i, result)| println!("F({}) = {}", i, result));
```

### Command-line application

#### Single

```bash
fib single 100
```

```bash
F(100) = 354224848179261915075
```

#### Range

```bash
fib range 6 10
```

```bash
F(6) = 8
F(7) = 13
F(8) = 21
F(9) = 34
F(10) = 55
```

## Performance

Specifications:

- 15-inch MacBook Air (2025)
- M4 Chip, 10C CPU, 10C GPU
- 32GB Unified Memory
- macOS Sequoia 15.4

| Single        | Computation Time |
|---------------|------------------|
| F(1,000)      | ~876ns           |
| F(10,000)     | ~8μs             |
| F(100,000)    | ~332μs           |
| F(1,000,000)  | ~10ms            |
| F(10,000,000) | ~326ms           |

| Range                | Computation Time |
|----------------------|------------------|
| F(0) -> F(1,000)     | ~110μs           |
| F(0) -> F(10,000)    | ~986μs           |
| F(0) -> F(100,000)   | ~45ms            |
| F(0) -> F(1,000,000) | ~33.9s           |

## Algorithm Details

### Single Fibonacci Number

For computing a single Fibonacci number, this implementation uses the fast doubling algorithm with logarithmic time complexity:

For even n: F(2k) = F(k) *(2*F(k+1) - F(k))

For odd n:  F(2k+1) = F(k+1)^2 + F(k)^2

This divide-and-conquer approach is vastly more efficient than naive recursive or iterative methods for large inputs.

### Fibonacci Range

The range implementation combines two approaches for optimal performance:

1. **Parallel processing**: Divides the requested range into optimal chunks based on available CPU threads
2. **Smart initialization**: Uses the fast doubling algorithm to efficiently find the starting values for each chunk
3. **Iterative calculation**: After finding starting values, computes subsequent Fibonacci numbers iteratively within each chunk

This hybrid approach provides excellent performance for generating sequences of consecutive Fibonacci numbers, especially for large ranges, by leveraging multi-core processing while maintaining mathematical efficiency.

## License

This project is licensed under the [MIT License](LICENSE).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
