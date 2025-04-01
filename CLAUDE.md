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

## Code Style
- **Imports**: Group by standard lib first, then external crates
- **Naming**: snake_case for functions/variables, CamelCase for types
- **Types**: Use BigUint for large numbers, u128 for inputs
- **Documentation**: All public functions must have doc comments
- **Error Handling**: Use match expressions, avoid unwrap()/expect() in production code
- **Testing**: Write tests in tests/ directory, use #[test] annotation
- **Formatting**: Follow rustfmt defaults, max line length 100 characters

## Performance
- Benchmark significant changes to fibonacci calculation algorithms
- Optimize for large numbers (> 10,000)