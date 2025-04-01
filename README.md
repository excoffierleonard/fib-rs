# fib-rs

[![Crates.io](https://img.shields.io/crates/v/fib-rs.svg)](https://crates.io/crates/fib-rs)
[![Documentation](https://docs.rs/fib-rs/badge.svg)](https://docs.rs/fib-rs)
[![MIT](https://img.shields.io/crates/l/fib-rs.svg)](LICENSE)

A highly optimized Fibonacci number calculator for Rust that efficiently computes arbitrarily large Fibonacci numbers.

## Features

- **Fast doubling algorithm**: Calculates Fibonacci numbers in O(log n) time
- **Handles massive inputs**: Compute Fibonacci numbers up to F(10,000,000) and beyond
- **Automatic optimization**: Uses primitive integers for smaller inputs and BigUint for larger ones
- **CLI application**: Simple command-line interface for quick calculations
- **Web API**: HTTP server with JSON interface for network-based calculations

## Installation

```bash
cargo add fib-rs
```

## Usage

### As a library

```rust
use fib_rs::fib;

// Calculate F(1000)
let input = 1000;
let result = fib_rs::fib(input);
println!("F({}) = {}", input, result);
```

### Command-line application

```bash
# Install only the CLI (this is the default)
cargo install fib-rs

# Or explicitly specify the CLI feature
cargo install fib-rs --features="cli" --no-default-features

# Calculate the 100th Fibonacci number
fib 100
```

### Web server

```bash
# Install only the web server
cargo install fib-rs --features="web" --no-default-features

# Run the server (defaults to port 8080)
fib-web

# Or specify a custom port with an environment variable
PORT=3000 fib-web
```

Then send a POST request:

```bash
curl --request POST \
     --url "http://localhost:8080/fib" \
     --header "Content-Type: application/json" \
     --data '{"n": 100}'
```

Response:

```json
{"F":"354224848179261915075"}
```

### Installation Options

The package provides these installation options:

- `--features="cli"`: Install only the command-line interface (default)
- `--features="web"`: Install only the web server
- `--features="cli,web"`: Install both CLI and web server
- `--no-default-features`: Disable default features (use with other feature flags)

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
| F(100,000) | ~10ms            |
| F(1,000,000)| ~329ms          |

## Algorithm Details

This implementation uses the fast doubling algorithm for Fibonacci numbers, which has logarithmic time complexity:

For even n: F(2k) = F(k) *(2*F(k+1) - F(k))
For odd n:  F(2k+1) = F(k+1)^2 + F(k)^2

This approach is vastly more efficient than naive recursive or iterative methods for large inputs.

## License

This project is licensed under the [MIT License](LICENSE).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
