use fib::fib;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::str::FromStr;

#[test]
fn correct_formula() {
    assert_eq!(fib(0), BigUint::zero());
    assert_eq!(fib(1), BigUint::one());
    assert_eq!(fib(10), BigUint::from(55u32));
    assert_eq!(fib(20), BigUint::from(6765u32));
}

#[test]
fn max_primitives_output() {
    assert_eq!(
        fib(186),
        BigUint::from(332825110087067562321196029789634457848u128)
    );
}

#[test]
fn beyond_u128_output_works() {
    assert_eq!(
        fib(187),
        BigUint::from_str("538522340430300790495419781092981030533").unwrap()
    );
}

#[test]
fn beyond_u8_input_works() {
    assert_eq!(
        fib(256),
        BigUint::from_str("141693817714056513234709965875411919657707794958199867").unwrap()
    );
}
