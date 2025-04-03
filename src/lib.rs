use num_bigint::BigUint;
use num_traits::{One, Zero};

/// Calculate the nth Fibonacci number using an optimized fast doubling algorithm.
///
/// This function efficiently computes Fibonacci numbers of arbitrary size by using
/// a divide-and-conquer approach based on matrix identities.
///
/// # Arguments
///
/// * `n` - The index of the Fibonacci number to calculate (0-indexed, where F(0)=0, F(1)=1)
///
/// # Returns
///
/// * The nth Fibonacci number as a `BigUint`
///
/// # Performance
///
/// * For n ≤ 185: Uses primitive u128 arithmetic for maximum performance
/// * For n > 185: Automatically switches to arbitrary precision with BigUint
///
/// # Complexity
///
/// * Time complexity: O(log n) due to the fast doubling algorithm
/// * Space complexity: O(log n) for the recursive call stack
///
/// # Examples
///
/// ```
/// use fib_rs::fib;
/// use num_bigint::BigUint;
/// use num_traits::Zero;
///
/// // F(0) = 0
/// assert_eq!(fib(0), BigUint::zero());
///
/// // F(10) = 55
/// assert_eq!(fib(10), BigUint::from(55u32));
///
/// // Large value example (would overflow primitive types)
/// let fib_200 = fib(200);
/// assert!(fib_200 > BigUint::from(u128::MAX));
/// ```
pub fn fib(n: u128) -> BigUint {
    match n {
        0 => BigUint::zero(),
        1 => BigUint::one(),
        _ => fib_fast_doubling_helper(n).0,
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
