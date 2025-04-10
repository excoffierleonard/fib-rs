use fib_rs::{fib, fib_range};

#[test]
fn test_fibonacci_identities() {
    // Test F(n+1) = F(n) + F(n-1) for various values
    for n in 2..1000 {
        assert_eq!(fib(n), fib(n - 1) + fib(n - 2));
    }
}

#[test]
fn test_range_consistency() {
    // Test individual vs range calculations match
    let range = 0..=1000;
    let range_results = fib_range(range.clone());

    for (i, n) in range.enumerate() {
        assert_eq!(range_results[i], fib(n));
    }
}
