use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::ops::RangeInclusive;

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

/// Generates Fibonacci numbers for indices in the given inclusive range.
///
/// # Arguments
///
/// * `range` - A range of indices (x..=y) for which to generate Fibonacci numbers.
///             The sequence will include the Fibonacci numbers at both indices x and y.
///
/// # Returns
///
/// * A `Vec<BigUint>` containing Fibonacci numbers for indices in the specified inclusive range.
///
/// # Examples
///
/// ```
/// use fib_rs::fib_sequence;
/// use num_bigint::BigUint;
/// use num_traits::{Zero, One};
///
/// // Generate Fibonacci numbers from index 3 to 10 (both 3 and 10 inclusive)
/// let fibs = fib_sequence(3..=10);
/// assert_eq!(fibs.len(), 8); // indices 3 to 10
/// // For example, F(3) should be 2:
/// assert_eq!(fibs[0], BigUint::from(2u32));
/// ```
pub fn fib_sequence(range: RangeInclusive<u128>) -> Vec<BigUint> {
    let &start = range.start();
    let &end = range.end();
    if end < start {
        return vec![];
    }

    let capacity = (end - start + 1) as usize;
    let mut result = Vec::with_capacity(capacity);

    // Get Fibonacci numbers at positions `start` and `start + 1` using the fast doubling helper.
    let (mut a, mut b) = fib_fast_doubling_helper(start);

    for _ in start..=end {
        result.push(a.clone());
        let next = a + &b;
        a = b;
        b = next;
    }
    result
}
