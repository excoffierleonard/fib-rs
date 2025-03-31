use num_bigint::BigUint;
use num_traits::{One, Zero};

/// Given a number n, return the nth Fibonacci number.
pub fn fib(n: u128) -> BigUint {
    match n {
        0 => BigUint::zero(),
        1 => BigUint::one(),
        n if n <= 186 => BigUint::from(fib_primitives(n as u8)),
        _ => fib_fast_doubling_helper(n).0,
    }
}

fn fib_primitives(n: u8) -> u128 {
    let mut prev = 0;
    let mut curr = 1;
    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    curr
}

fn fib_fast_doubling_helper(n: u128) -> (BigUint, BigUint) {
    match n {
        0 => (Zero::zero(), One::one()),
        _ => {
            let (fk, fk1) = fib_fast_doubling_helper(n / 2);

            let two_fk1 = &fk1 << 1;
            // Assuming standard Fibonacci sequence where F(k) <= 2*F(k+1)
            let term = two_fk1 - &fk;
            let f2k = &fk * term;

            let fk_squared = &fk * &fk;
            let fk1_squared = &fk1 * &fk1;
            let f2k1 = fk_squared + fk1_squared;

            match n % 2 {
                0 => (f2k, f2k1),
                _ => {
                    let f2k2 = &f2k1 + &f2k;
                    (f2k1, f2k2)
                }
            }
        }
    }
}
