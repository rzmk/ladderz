use std::collections::HashSet;

/// Finds all factor pairs for a given positive integer.
/// 
/// # Challenge
/// 
/// Write a program that finds all the factor pairs for a given number `n`.
/// 
/// # Description
/// 
/// Generates a `HashSet` of factor pairs for a given positive integer `n`.
///
/// This function calculates and returns a `HashSet` containing all unique factor pairs
/// of the input positive integer `n`. A factor pair is a pair of positive integers
/// `(a, b)` where `a` and `b` are both factors of `n` (i.e., `a * b == n`).
///
/// # Arguments
///
/// * `n` - The positive integer for which factor pairs are to be calculated.
///
/// # Returns
///
/// A `HashSet` containing all unique factor pairs of the input integer `n`.
///
/// # Examples
///
/// ```rust
/// use std::collections::HashSet;
/// use ladderz::pre_algebra::unit1::get_factor_pairs;
///
/// fn main() {
///     let result_pairs = get_factor_pairs(12);
///     let expected_pairs: HashSet<(u32, u32)> = [(1, 12), (2, 6), (3, 4)].into();
///     assert_eq!(result_pairs, expected_pairs);
/// }
/// ```
///
/// # Note
///
/// This function calculates factor pairs by iterating through positive integers from 1 to `n`
/// (inclusive) and checking if they divide `n` evenly. If they do, a factor pair `(a, b)` is
/// added to the `HashSet`. The function ensures that factor pairs are unique, so `(a, b)` and
/// `(b, a)` will not both appear in the set.
pub fn get_factor_pairs(n: u32) -> HashSet<(u32, u32)> {
    let mut factor_pairs: HashSet<(u32, u32)> = HashSet::new();

    for num in 1..n + 1 {
        let dividend: u32 = n;
        let divisor: u32 = num;
        let quotient: u32 = dividend / num;
        let remainder: u32 = dividend % num;
        if remainder == 0 && !factor_pairs.contains(&(quotient, divisor)) {
            factor_pairs.insert((divisor, quotient));
        }
    }

    factor_pairs
}

/// Finds all factors of a given positive integer.
/// 
/// # Challenge
/// 
/// Write a program that finds all the factors of a given number. Assume that `n` is a positive integer greater than or equal to 1.
///
/// # Description
/// 
/// Generates a `HashSet` of factors for a given positive integer `n`.
///
/// This function calculates and returns a `HashSet` containing all unique factors
/// of the input positive integer `n`. A factor of `n` is a positive integer `a` where
/// `n` is evenly divisible by `a` (i.e., `n % a == 0`).
///
/// # Arguments
///
/// * `n` - The positive integer for which factors are to be calculated.
///
/// # Returns
///
/// A `HashSet` containing all unique factors of the input integer `n`.
///
/// # Examples
///
/// ```rust
/// use std::collections::HashSet;
/// use ladderz::pre_algebra::unit1::get_factors;
///
/// fn main() {
///     let result_factors = get_factors(16);
///     let expected_factors: HashSet<u32> = [1, 2, 4, 8, 16].into();
///     assert_eq!(result_factors, expected_factors);
/// }
/// ```
///
/// # Note
///
/// This function calculates factors by iterating through positive integers from 1 to `n`
/// (inclusive) and checking if they divide `n` evenly. If they do, the factor is added to
/// the `HashSet`. The function ensures that factors are unique, so duplicates are not added.
pub fn get_factors(n: u32) -> HashSet<u32> {
    let mut factors: HashSet<u32> = HashSet::new();

    for num in 1..n+1 {
        if n % num == 0 {
            factors.insert(num);
        }
    }
    factors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_factor_pairs() {
        let result: HashSet<(u32, u32)> = get_factor_pairs(1);
        let expected: HashSet<(u32, u32)> = [(1, 1)].into();
        assert_eq!(result, expected);

        let result_2: HashSet<(u32, u32)> = get_factor_pairs(16);
        let expected_2: HashSet<(u32, u32)> = [(1, 16), (2, 8), (4, 4)].into();
        assert_eq!(result_2, expected_2);
    }

    #[test]
    fn test_get_factors() {
        let result: HashSet<u32> = get_factors(1);
        let expected: HashSet<u32> = [1].into();
        assert_eq!(result, expected);

        let result_2: HashSet<u32> = get_factors(16);
        let expected_2: HashSet<u32> = [1, 2, 4, 8, 16].into();
        assert_eq!(result_2, expected_2);
    }
}
