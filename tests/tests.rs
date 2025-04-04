use fib_rs::{fib, fib_sequence};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::str::FromStr;

#[test]
fn correct_fib_formula() {
    // The formula is respected
    assert_eq!(fib(0), BigUint::zero());
    assert_eq!(fib(1), BigUint::one());
    assert_eq!(fib(10), BigUint::from_str("55").unwrap());
    assert_eq!(fib(20), BigUint::from_str("6765").unwrap());
    // Beyond u128 outputs are correct
    assert_eq!(
        fib(187),
        BigUint::from_str("538522340430300790495419781092981030533").unwrap()
    );
    // Beyond u8 inputs produces correct output
    assert_eq!(
        fib(256),
        BigUint::from_str("141693817714056513234709965875411919657707794958199867").unwrap()
    );
}

#[test]
fn correct_sequence_generation() {
    let fib_seq_1 = fib_sequence(0..=100);
    assert_eq!(fib_seq_1.len(), 101);
    assert_eq!(fib_seq_1[0], BigUint::zero());
    assert_eq!(fib_seq_1[1], BigUint::one());
    assert_eq!(fib_seq_1[10], BigUint::from_str("55").unwrap());
    assert_eq!(fib_seq_1[20], BigUint::from_str("6765").unwrap());

    let fib_seq_2 = fib_sequence(100..=300);
    assert_eq!(fib_seq_2.len(), 201);
    assert_eq!(
        fib_seq_2[0],
        BigUint::from_str("354224848179261915075").unwrap()
    ); // Supposed to be fib 100
    assert_eq!(
        fib_seq_2[1],
        BigUint::from_str("573147844013817084101").unwrap()
    ); // Supposed to be fib 101
    assert_eq!(
        fib_seq_2[156],
        BigUint::from_str("141693817714056513234709965875411919657707794958199867").unwrap()
    );
}

// Loop over all the Fibonacci numbers from 0 to 256 (u8+1) to ensure the function works with a variety of numbers
#[test]
fn loop_over_fibonacci() {
    for i in 0..=256 {
        fib(i);
    }
}

// Test with various ranges
#[test]
fn loop_over_fibonacci_sequence() {
    let ranges = vec![
        0..=100,
        50..=150,
        100..=200,
        150..=256,
        200..=256,
        0..=256,
        0..=1000,
        1000..=2000,
    ];

    for range in ranges {
        fib_sequence(range);
    }
}
