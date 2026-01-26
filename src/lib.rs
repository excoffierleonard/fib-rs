//! # fib-rs
//!
//! A highly optimized Fibonacci number calculator for Rust that efficiently computes
//! arbitrarily large Fibonacci numbers.
//!
//! ## Features
//!
//! - **Fast doubling algorithm**: Calculates Fibonacci numbers in O(log n) time
//! - **Handles massive inputs**: Compute Fibonacci numbers up to F(10,000,000) and beyond
//! - **Range calculation**: Generate sequences of consecutive Fibonacci numbers with parallel processing
//! - **BigUint support**: Uses arbitrary precision integers for handling large Fibonacci numbers
//!
//! ## Examples
//!
//! ```
//! use fib_rs::Fib;
//! use num_bigint::BigUint;
//! use num_traits::Zero;
//!
//! // Calculate a single Fibonacci number
//! let n = 100;
//! let result = Fib::single(n);
//! assert!(result > BigUint::zero());
//!
//! // Calculate a range of Fibonacci numbers (F(3) through F(10))
//! let start = 3;
//! let end = 10;
//! let results = Fib::range(start, end);
//! assert_eq!(results.len(), (end - start + 1) as usize);
//! ```
//!
//! ## Algorithm Details
//!
//! ### Single Fibonacci Number
//!
//! For computing a single Fibonacci number, this implementation uses the fast doubling algorithm
//! with logarithmic time complexity:
//!
//! - For even n: F(2k) = F(k) * (2*F(k+1) - F(k))
//! - For odd n:  F(2k+1) = F(k+1)^2 + F(k)^2
//!
//! ### Fibonacci Range
//!
//! The range implementation combines three approaches for optimal performance:
//!
//! 1. **Parallel processing**: Divides the requested range into optimal chunks based on available CPU threads
//! 2. **Smart initialization**: Uses the fast doubling algorithm to efficiently find the starting values for each chunk
//! 3. **Iterative calculation**: After finding starting values, computes subsequent Fibonacci numbers iteratively within each chunk

use std::{
    cmp::{max, min},
    iter::from_fn,
    mem::{replace, take},
};

use num_bigint::BigUint;
use num_traits::{One, Zero};
use rayon::{current_num_threads, prelude::*};

/// Type alias for the result of the fast doubling algorithm
///
/// Represents a pair of consecutive Fibonacci numbers (F(n), F(n+1))
type FibPair = (BigUint, BigUint);

/// A utility struct for computing Fibonacci numbers efficiently.
///
/// This struct provides static methods for calculating Fibonacci numbers using
/// highly optimized algorithms. It supports both single Fibonacci number calculation
/// and generating ranges of consecutive Fibonacci numbers.
///
/// The implementation uses a fast doubling algorithm for O(log n) time complexity
/// and leverages parallel processing for range calculations to maximize performance.
///
/// Uses `BigUint` for arbitrary precision, ensuring correct results for extremely large
/// Fibonacci numbers.
pub struct Fib;

impl Fib {
    /// Calculate the nth Fibonacci number using an optimized fast doubling algorithm.
    ///
    /// This method efficiently computes Fibonacci numbers of arbitrary size by using
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
    /// use fib_rs::Fib;
    /// use num_bigint::BigUint;
    /// use num_traits::Zero;
    ///
    /// assert_eq!(Fib::single(0), BigUint::zero()); // F(0) = 0
    /// assert_eq!(Fib::single(10), BigUint::from(55u32)); // F(10) = 55
    /// assert!(Fib::single(200) > BigUint::from(u128::MAX)); // Large value example (would overflow primitive types)
    /// ```
    pub fn single(n: u128) -> BigUint {
        match n {
            0 => BigUint::zero(),
            1 => BigUint::one(),
            _ => Self::fib_fast_doubling_helper(n).0,
        }
    }

    /// Helper function for the fast doubling algorithm.
    ///
    /// This function implements the recursive divide-and-conquer approach for computing
    /// Fibonacci numbers using the fast doubling method. It returns a pair of consecutive
    /// Fibonacci numbers (F(n), F(n+1)) to enable the recursion.
    ///
    /// The algorithm is based on the following mathematical identities:
    /// - For even n: F(2k) = F(k) * (2*F(k+1) - F(k))
    /// - For odd n:  F(2k+1) = F(k+1)^2 + F(k)^2
    ///
    /// # Arguments
    ///
    /// * `n` - The index of the Fibonacci number to calculate
    ///
    /// # Returns
    ///
    /// * A tuple (F(n), F(n+1)) containing the nth and (n+1)th Fibonacci numbers
    ///
    /// # Time Complexity
    ///
    /// * O(log n) due to the recursive divide-and-conquer approach
    fn fib_fast_doubling_helper(n: u128) -> FibPair {
        if n == 0 {
            return (BigUint::zero(), BigUint::one());
        }

        // Calculate F(k) and F(k+1) where k = floor(n/2)
        let (fk, fk1) = Self::fib_fast_doubling_helper(n / 2);

        // Calculate F(2k) and F(2k+1)
        let two_fk1 = &fk1 << 1; // Efficiently multiply by 2 using bit shift
        let term = &two_fk1 - &fk;
        let f2k = &fk * &term; // F(2k) = F(k) * (2*F(k+1) - F(k))
        let f2k1 = &fk * &fk + &fk1 * &fk1; // F(2k+1) = F(k)^2 + F(k+1)^2

        // Return appropriate pair based on whether n is even or odd
        // Already internally does a bitwise operation
        if n.is_multiple_of(2) {
            (f2k, f2k1)
        } else {
            let f2k2 = &f2k1 + &f2k;
            (f2k1, f2k2)
        }
    }

    /// Generates Fibonacci numbers for indices in the given inclusive range.
    ///
    /// This method efficiently computes a sequence of consecutive Fibonacci numbers
    /// using parallel processing for improved performance. It divides the requested range
    /// into optimal chunks based on the available CPU threads, calculates each chunk in
    /// parallel, and then combines the results.
    ///
    /// The implementation uses a hybrid approach that:
    /// 1. Uses the fast doubling algorithm to efficiently find the starting values for each chunk
    /// 2. Computes subsequent Fibonacci numbers iteratively within each chunk
    /// 3. Processes chunks in parallel using the Rayon library
    ///
    /// # Arguments
    ///
    /// * `start` - The starting index of the range
    /// * `end` - The ending index of the range (inclusive)
    ///
    /// # Returns
    ///
    /// * A `Vec<BigUint>` containing ordered Fibonacci numbers for indices in the specified inclusive range.
    ///
    /// # Complexity
    ///
    /// * Time complexity: Approximately O(n/t + log(start)) where n is the range size and t is the number of threads.
    /// * Space complexity: O(n) for storing the Fibonacci numbers in a vector.
    ///
    /// # Performance
    ///
    /// Performance scales with the number of available CPU cores. For large ranges, the
    /// parallelization provides significant speedup compared to sequential calculation.
    ///
    /// # Examples
    ///
    /// ```
    /// use fib_rs::Fib;
    /// use num_bigint::BigUint;
    /// use num_traits::{Zero, One};
    ///
    /// // Generate Fibonacci numbers from index 3 to 10 (both 3 and 10 inclusive)
    /// let fibs = Fib::range(3, 10);
    ///
    /// assert_eq!(fibs.len(), 8); // indices 3 to 10
    /// assert_eq!(fibs[0], BigUint::from(2u32)); // F(3) = 2
    /// assert_eq!(fibs[7], BigUint::from(55u32)); // F(10) = 55
    /// ```
    ///
    /// For large ranges, the parallel implementation provides significant performance improvements:
    ///
    /// ```
    /// use fib_rs::Fib;
    ///
    /// // Generate 10,000 consecutive Fibonacci numbers efficiently
    /// let large_range = Fib::range(1000, 10999);
    /// assert_eq!(large_range.len(), 10000);
    /// ```
    pub fn range(start: u128, end: u128) -> Vec<BigUint> {
        // Validate input range
        if end < start {
            return Vec::new();
        }

        // Calculate total number of Fibonacci numbers to generate
        let total_count = (end - start + 1) as usize;

        // Determine optimal chunk size for parallelization based on available CPU threads
        // This balances parallelism with the overhead of creating too many small chunks
        let num_threads = current_num_threads();
        let chunk_size = max(1, total_count / num_threads);

        // Calculate number of chunks with ceiling division to ensure we cover the entire range
        let num_chunks = total_count.div_ceil(chunk_size);

        // Create chunk boundaries for parallel processing
        // Each chunk represents a subrange of consecutive Fibonacci numbers
        let chunks: Vec<_> = (0..num_chunks)
            .map(|i| {
                let chunk_start = start + (i as u128) * (chunk_size as u128);
                let chunk_end = min(chunk_start + (chunk_size as u128) - 1, end);
                (chunk_start, chunk_end)
            })
            .collect();

        // Process each chunk in parallel using Rayon's parallel iterator
        // Each thread calculates a portion of the Fibonacci sequence independently
        chunks
            .par_iter()
            .flat_map_iter(|&(chunk_start, chunk_end)| {
                let chunk_size = (chunk_end - chunk_start + 1) as usize;

                // Use the fast doubling algorithm to efficiently find the starting values
                // for this chunk. This is a key optimization for chunk initialization.
                let (mut a, mut b) = Self::fib_fast_doubling_helper(chunk_start);
                let mut remaining = chunk_size;

                // Compute the chunk iteratively using the recurrence relation:
                // F(n+2) = F(n+1) + F(n)
                // This is more efficient than using the fast doubling algorithm for each number
                from_fn(move || {
                    if remaining == 0 {
                        return None;
                    }
                    let next = &a + &b;
                    let out = take(&mut a);
                    a = replace(&mut b, next);
                    remaining -= 1;
                    Some(out)
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn correct_fib_formula() {
        // The formula is respected
        assert_eq!(Fib::single(0), BigUint::zero());
        assert_eq!(Fib::single(1), BigUint::one());
        assert_eq!(Fib::single(10), BigUint::from_str("55").unwrap());
        assert_eq!(Fib::single(20), BigUint::from_str("6765").unwrap());
        // Beyond u128 outputs are correct
        assert_eq!(
            Fib::single(187),
            BigUint::from_str("538522340430300790495419781092981030533").unwrap()
        );
        // Beyond u8 inputs produces correct output
        assert_eq!(
            Fib::single(256),
            BigUint::from_str("141693817714056513234709965875411919657707794958199867").unwrap()
        );
    }

    #[test]
    fn correct_sequence_generation() {
        let fib_seq_1 = Fib::range(0, 100);
        assert_eq!(fib_seq_1.len(), 101);
        assert_eq!(fib_seq_1[0], BigUint::zero());
        assert_eq!(fib_seq_1[1], BigUint::one());
        assert_eq!(fib_seq_1[10], BigUint::from_str("55").unwrap());
        assert_eq!(fib_seq_1[20], BigUint::from_str("6765").unwrap());

        let fib_seq_2 = Fib::range(100, 300);
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
        let fib_seq = Fib::range(0, 100);
        // Start from the third element since the first three results are 0, 1, 1
        for i in 3..fib_seq.len() {
            assert!(fib_seq[i] > fib_seq[i - 1]);
        }
    }

    // Loop over all the Fibonacci numbers to ensure the function does not panic over a variety of inputs
    #[test]
    fn loop_over_fibonacci() {
        for i in 0..=1000 {
            Fib::single(i);
        }
    }

    // Test with various ranges
    #[test]
    fn loop_over_fibonacci_ranges() {
        let ranges = vec![
            (0, 100),
            (50, 150),
            (100, 200),
            (150, 250),
            (200, 300),
            (350, 450),
            (400, 500),
            (450, 550),
            (500, 600),
            (550, 650),
            (600, 700),
            (650, 750),
            (700, 800),
            (750, 850),
            (800, 900),
            (850, 950),
            (900, 1000),
        ];

        for (start, end) in ranges {
            Fib::range(start, end);
        }
    }
}
