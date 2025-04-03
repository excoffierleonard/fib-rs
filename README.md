# fib-rs

[![Crates.io](https://img.shields.io/crates/v/fib-rs)](https://crates.io/crates/fib-rs)
[![Documentation](https://img.shields.io/docsrs/fib-rs)](https://docs.rs/fib-rs)
[![MIT](https://img.shields.io/crates/l/fib-rs)](LICENSE)

A highly optimized Fibonacci number calculator for Rust that efficiently computes arbitrarily large Fibonacci numbers.

## Features

- **Fast doubling algorithm**: Calculates Fibonacci numbers in O(log n) time
- **Handles massive inputs**: Compute Fibonacci numbers up to F(10,000,000) and beyond
- **Automatic optimization**: Uses primitive integers for smaller inputs and BigUint for larger ones
- **CLI application**: Simple command-line interface for quick calculations

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
use fib_rs::fib;

// Calculate F(100)
let input = 100;
let result = fib_rs::fib(input);
println!("F({}) = {}", input, result);
```

### Command-line application

```bash
# Calculate the 100th Fibonacci number
fib 100
```

## Performance

The implementation uses a fast doubling algorithm with the following optimizations:

- For n ≤ 185: Uses primitive u128 integers
- For n > 185: Uses BigUint for arbitrary precision

Benchmark results:

Specifications:

- 15-inch MacBook Air (2025)
- M4 Chip, 10C CPU, 10C GPU
- 32GB Unified Memory
- macOS Sequoia 15.4

| Input Size | Computation Time |
|------------|------------------|
| F(1,000)   | ~876ns           |
| F(10,000)  | ~8μs             |
| F(100,000) | ~332μs           |
| F(1,000,000)| ~10ms           |
| F(10,000,000)| ~326ms         |

## Algorithm Details

This implementation uses the fast doubling algorithm for Fibonacci numbers, which has logarithmic time complexity:

For even n: F(2k) = F(k) *(2*F(k+1) - F(k))
For odd n:  F(2k+1) = F(k+1)^2 + F(k)^2

This approach is vastly more efficient than naive recursive or iterative methods for large inputs.

## License

This project is licensed under the [MIT License](LICENSE).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
