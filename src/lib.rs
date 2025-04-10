use std::{
    cmp::{max, min},
    mem::replace,
    ops::RangeInclusive,
};

use num_bigint::BigUint;
use num_traits::{One, Zero};
use rayon::{current_num_threads, prelude::*};

/// Type alias for the result of the fast doubling algorithm
type FibPair = (BigUint, BigUint);

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
/// assert_eq!(fib(0), BigUint::zero()); // F(0) = 0
/// assert_eq!(fib(10), BigUint::from(55u32)); // F(10) = 55
/// assert!(fib(200) > BigUint::from(u128::MAX)); // Large value example (would overflow primitive types)
/// ```
pub fn fib(n: u128) -> BigUint {
    match n {
        0 => BigUint::zero(),
        1 => BigUint::one(),
        _ => fib_fast_doubling_helper(n).0,
    }
}

fn fib_fast_doubling_helper(n: u128) -> FibPair {
    if n == 0 {
        return (BigUint::zero(), BigUint::one());
    }

    // Calculate F(k) and F(k+1) where k = floor(n/2)
    let (fk, fk1) = fib_fast_doubling_helper(n / 2);

    // Calculate F(2k) and F(2k+1)
    let two_fk1 = &fk1 << 1;
    let term = &two_fk1 - &fk;
    let f2k = &fk * &term;
    let f2k1 = &fk * &fk + &fk1 * &fk1;

    // Return appropriate pair based on whether n is even or odd
    if n % 2 == 0 {
        (f2k, f2k1)
    } else {
        let f2k2 = &f2k1 + &f2k;
        (f2k1, f2k2)
    }
}

/// Generates Fibonacci numbers for indices in the given inclusive range.
///
/// # Arguments
///
/// * `range` - A range of indices (x..=y) for which to generate Fibonacci numbers.
///   The sequence will include the Fibonacci numbers at both indices x and y.
///
/// # Returns
///
/// * A `Vec<BigUint>` containing ordered Fibonacci numbers for indices in the specified inclusive range.
///
/// # Complexity
///
/// * Time complexity: O(n) for generating Fibonacci numbers in the range.
/// * Space complexity: O(n) for storing the Fibonacci numbers in a vector.
///
/// # Examples
///
/// ```
/// use fib_rs::fib_range;
/// use num_bigint::BigUint;
/// use num_traits::{Zero, One};
///
/// // Generate Fibonacci numbers from index 3 to 10 (both 3 and 10 inclusive)
/// let fibs = fib_range(3..=10);
///
/// assert_eq!(fibs.len(), 8); // indices 3 to 10
/// assert_eq!(fibs[0], BigUint::from(2u32)); // F(3) = 2
/// assert_eq!(fibs[7], BigUint::from(55u32)); // F(10) = 55
/// ```
pub fn fib_range(range: RangeInclusive<u128>) -> Vec<BigUint> {
    let start = *range.start();
    let end = *range.end();

    if end < start {
        return Vec::new();
    }

    let total_count = (end - start + 1) as usize;

    // Determine chunk size for parallelization
    let num_threads = current_num_threads();
    let chunk_size = max(1, total_count / num_threads);

    // Calculate number of chunks with ceiling division
    let num_chunks = total_count.div_ceil(chunk_size);

    // Create chunk boundaries
    let chunks: Vec<_> = (0..num_chunks)
        .map(|i| {
            let chunk_start = start + (i as u128) * (chunk_size as u128);
            let chunk_end = min(chunk_start + (chunk_size as u128) - 1, end);
            (chunk_start, chunk_end)
        })
        .collect();

    // Process each chunk in parallel
    let results: Vec<Vec<BigUint>> = chunks
        .par_iter()
        .map(|&(chunk_start, chunk_end)| {
            let chunk_size = (chunk_end - chunk_start + 1) as usize;
            let mut result = Vec::with_capacity(chunk_size);

            // Get starting Fibonacci numbers for this chunk
            let (mut a, mut b) = fib_fast_doubling_helper(chunk_start);

            // Add the first value
            result.push(a.clone());

            // Compute the rest of the chunk iteratively
            for _ in 1..chunk_size {
                let next = &a + &b;
                a = replace(&mut b, next);
                result.push(a.clone());
            }

            result
        })
        .collect();

    // Combine results efficiently
    let mut result = Vec::with_capacity(total_count);
    for chunk in results {
        result.extend(chunk);
    }

    result
}
