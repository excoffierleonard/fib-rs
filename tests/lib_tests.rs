use fib_rs::Fib;

#[test]
fn test_fibonacci_identities() {
    // Test F(n+1) = F(n) + F(n-1) for various values
    for n in 2..1000 {
        assert_eq!(Fib::single(n), Fib::single(n - 1) + Fib::single(n - 2));
    }
}

#[test]
fn test_range_consistency() {
    // Test individual vs range calculations match
    let range = 0..=1000;
    let range_results = Fib::range(*range.start(), *range.end());

    for (i, n) in range.enumerate() {
        assert_eq!(range_results[i], Fib::single(n));
    }
}
