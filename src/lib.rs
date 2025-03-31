use num_bigint::BigUint;
use num_traits::{One, Zero};

/// Given a number n, return the nth Fibonacci number.
pub fn fib(n: u128) -> BigUint {
    // TODO: Implement Matrix exponentiation
    match n {
        0 => BigUint::zero(),
        1 => BigUint::one(),
        n if n <= 185 => BigUint::from(fib_fast_doubling_primitive_helper(n).0),
        _ => fib_fast_doubling_helper(n).0,
    }
}

fn fib_fast_doubling_primitive_helper(n: u128) -> (u128, u128) {
    match n {
        0 => (0, 1), // Base case F(0)=0, F(1)=1
        _ => {
            // Recursively get F(k), F(k+1) where k = n / 2
            let (fk, fk1) = fib_fast_doubling_primitive_helper(n / 2);

            // Calculate F(2k) = F(k) * (2*F(k+1) - F(k))
            // Operations are standard u128 arithmetic. Overflow is not checked here.
            let two_fk1 = fk1 << 1; // F(k+1) * 2
            let term = two_fk1 - fk; // 2*F(k+1) - F(k)
            let f2k = fk * term;

            // Calculate F(2k+1) = F(k+1)^2 + F(k)^2
            let fk_squared = fk * fk;
            let fk1_squared = fk1 * fk1;
            let f2k1 = fk_squared + fk1_squared;

            match n % 2 {
                0 => (f2k, f2k1), // n is even (n=2k), return (F(2k), F(2k+1))
                _ => {
                    // n is odd (n=2k+1), return (F(2k+1), F(2k+2))
                    let f2k2 = f2k1 + f2k; // F(2k+2) = F(2k+1) + F(2k)
                    (f2k1, f2k2)
                }
            }
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limits() {
        // Up to u128 calculations
        for i in 2..=185 {
            fib_fast_doubling_primitive_helper(i);
        }
        // Beyond u8 inputs
        for i in 186..=256 {
            fib_fast_doubling_helper(i);
        }
    }

    #[test]
    #[should_panic]
    fn limit_break_u128_calculations() {
        fib_fast_doubling_primitive_helper(186);
    }
}
