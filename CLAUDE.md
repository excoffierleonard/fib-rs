# fib-rs Development Guidelines

## Commands

- Build: `cargo build`
- Run tests: `cargo test`
- Run single test: `cargo test <test_name>` (e.g., `cargo test correct_formula`)
- Run benchmarks: `cargo bench`
- Check format: `cargo fmt -- --check`
- Lint: `cargo clippy -- -D warnings`
- Run CLI: `cargo run --bin fib <number>`
- Run web server: `cargo run --bin fib-web`
- Build docs: `cargo doc --no-deps --open`

## Code Style

- **Imports**: Group standard lib first, then external crates, alphabetically within each group
- **Naming**: snake_case for functions/variables, CamelCase for types
- **Types**: Use BigUint for large numbers (n > 185), u128 for smaller inputs
- **Documentation**: All public functions must have doc comments with examples, arguments, returns sections
- **Error Handling**: Use match expressions, avoid unwrap()/expect() in production code
- **Testing**: Write integration tests in tests/ directory, unit tests in #[cfg(test)] modules
- **Formatting**: Follow rustfmt defaults, max line length 100 characters
- **Feature Flags**: Enable only necessary features for each binary/use case

## Performance

- Benchmark significant changes to fibonacci calculation algorithms
- Optimize for large numbers (> 10,000)
- Establish performance thresholds for various target platforms
