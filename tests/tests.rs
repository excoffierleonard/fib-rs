use fib::fib;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::str::FromStr;

#[test]
fn correct_formula() {
    assert_eq!(fib(0), BigUint::zero());
    assert_eq!(fib(1), BigUint::one());
    assert_eq!(fib(10), BigUint::from_str("55").unwrap());
    assert_eq!(fib(20), BigUint::from_str("6765").unwrap());
}

#[test]
fn primitives_output_limits() {
    assert_eq!(fib(93), BigUint::from_str("12200160415121876738").unwrap());
    assert_eq!(
        fib(186),
        BigUint::from_str("332825110087067562321196029789634457848").unwrap()
    );
    assert_eq!(
        fib(187),
        BigUint::from_str("538522340430300790495419781092981030533").unwrap()
    );
}

#[test]
fn primitives_input_limits() {
    assert_eq!(
        fib(255),
        BigUint::from_str("87571595343018854458033386304178158174356588264390370").unwrap()
    );
    assert_eq!(
        fib(256),
        BigUint::from_str("141693817714056513234709965875411919657707794958199867").unwrap()
    );
}
