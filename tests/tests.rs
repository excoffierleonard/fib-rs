use fib_rs::fib;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::str::FromStr;

#[test]
fn correct_formula() {
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

// Loop over all the Fibonacci numbers from 0 to 256 (u8+1) to ensure the function works with a variety of numbers
#[test]
fn loop_over_fibonacci() {
    for i in 0..=256 {
        fib(i);
    }
}
