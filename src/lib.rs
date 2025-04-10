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

#[cfg(test)]
mod tests {
    use super::*;
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
        let fib_seq_1 = fib_range(0..=100);
        assert_eq!(fib_seq_1.len(), 101);
        assert_eq!(fib_seq_1[0], BigUint::zero());
        assert_eq!(fib_seq_1[1], BigUint::one());
        assert_eq!(fib_seq_1[10], BigUint::from_str("55").unwrap());
        assert_eq!(fib_seq_1[20], BigUint::from_str("6765").unwrap());

        let fib_seq_2 = fib_range(100..=300);
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

    #[test]
    fn ordering_check() {
        // Check that the Fibonacci sequence is strictly increasing
        let fib_seq = fib_range(0..=100);
        // Start from the third element since the first three results are 0, 1, 1
        for i in 3..fib_seq.len() {
            assert!(fib_seq[i] > fib_seq[i - 1]);
        }
    }

    // Loop over all the Fibonacci numbers to ensure the function does not panic over a variety of inputs
    #[test]
    fn loop_over_fibonacci() {
        for i in 0..=1000 {
            fib(i);
        }
    }

    // Test with various ranges
    #[test]
    fn loop_over_fibonacci_ranges() {
        let ranges = vec![
            0..=100,
            50..=150,
            100..=200,
            150..=250,
            200..=300,
            350..=450,
            400..=500,
            450..=550,
            500..=600,
            550..=650,
            600..=700,
            650..=750,
            700..=800,
            750..=850,
            800..=900,
            850..=950,
            900..=1000,
        ];

        for range in ranges {
            fib_range(range);
        }
    }
}
