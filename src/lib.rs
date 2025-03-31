use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::str::FromStr;

/// Given a number n, return the nth Fibonacci number.
pub fn fib(n: u8) -> BigUint {
    match n {
        0 => BigUint::zero(),
        1 => BigUint::one(),
        n if n <= 186 => BigUint::from(fib_primitives(n)),
        _ => fib_beyond_max_primitives(n),
    }
}

pub fn fib_primitives(n: u8) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut prev = 0;
            let mut curr = 1;
            for _ in 2..=n {
                let next = prev + curr;
                prev = curr;
                curr = next;
            }
            curr
        }
    }
}

pub fn fib_beyond_max_primitives(n: u8) -> BigUint {
    match n {
        0 => BigUint::zero(),
        1 => BigUint::one(),
        _ => {
            let mut prev = BigUint::zero();
            let mut curr = BigUint::one();
            for _ in 2..=n {
                let next = &prev + &curr;
                prev = curr;
                curr = next;
            }
            curr
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_formula() {
        assert_eq!(fib_primitives(0), 0);
        assert_eq!(fib_primitives(1), 1);
        assert_eq!(fib_primitives(10), 55);
        assert_eq!(fib_primitives(20), 6765);

        assert_eq!(fib_beyond_max_primitives(0), BigUint::zero());
        assert_eq!(fib_beyond_max_primitives(1), BigUint::one());
        assert_eq!(fib_beyond_max_primitives(10), BigUint::from(55u32));
        assert_eq!(fib_beyond_max_primitives(20), BigUint::from(6765u32));

        assert_eq!(fib(0), BigUint::zero());
        assert_eq!(fib(1), BigUint::one());
        assert_eq!(fib(10), BigUint::from(55u32));
        assert_eq!(fib(20), BigUint::from(6765u32));
    }

    #[test]
    fn max_primitives_input() {
        assert_eq!(fib_primitives(186), 332825110087067562321196029789634457848);
        assert_eq!(
            fib_beyond_max_primitives(186),
            BigUint::from(332825110087067562321196029789634457848u128)
        );
        assert_eq!(
            fib(186),
            BigUint::from(332825110087067562321196029789634457848u128)
        );
    }

    #[test]
    #[should_panic]
    fn more_than_max_primitives_output_panics() {
        fib_primitives(187);
    }

    #[test]
    fn beyond_u128_works() {
        assert_eq!(
            fib_beyond_max_primitives(187),
            BigUint::from_str("538522340430300790495419781092981030533").unwrap()
        );
        assert_eq!(
            fib_beyond_max_primitives(255),
            BigUint::from_str("87571595343018854458033386304178158174356588264390370").unwrap()
        );
        assert_eq!(
            fib(187),
            BigUint::from_str("538522340430300790495419781092981030533").unwrap()
        );
        assert_eq!(
            fib(255),
            BigUint::from_str("87571595343018854458033386304178158174356588264390370").unwrap()
        );
    }
}
