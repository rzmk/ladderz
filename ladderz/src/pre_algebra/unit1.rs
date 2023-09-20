use std::collections::HashSet;

/// Finds all factor pairs for a positive integer `n`.
///
/// # Challenge
///
/// Write a program that finds all the factor pairs for a number `n`.
///
/// # Description
///
/// Generates a `HashSet` of factor pairs for a positive integer `n`.
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

/// Finds all factors of a positive integer `n`.
///
/// # Challenge
///
/// Write a program that finds all the factors of a number. Assume that `n` is a positive integer greater than or equal to 1.
///
/// # Description
///
/// Generates a `HashSet` of factors for a positive integer `n`.
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

    for num in 1..n + 1 {
        if n % num == 0 {
            factors.insert(num);
        }
    }
    factors
}

/// Checks if a positive integer `x` is a factor of another positive integer `y`.
///
/// # Challenge
///
/// Write a program that determines whether one positive integer is a factor of another.
///
/// # Description
///
/// Checks if a positive integer `x` is a factor of another positive integer `y`.
///
/// A factor of `y` is a positive integer `x` where `y` is evenly divisible by `x` (i.e., `y % x == 0`).
///
/// # Arguments
///
/// * `x` - The positive integer to determine whether it is a factor of `y` or not.
/// * `y` - The positive integer for which the factor check of `x` is performed.
///
/// # Returns
///
/// `true` if `x` is a factor of `y`, `false` otherwise.
///
/// # Examples
///
/// ```rust
/// use ladderz::pre_algebra::unit1::is_factor;
///
/// fn main() {
///     assert!(is_factor(2, 16)); // 2 is a factor of 16
///     assert!(!is_factor(3, 16)); // 3 is not a factor of 16
/// }
/// ```
///
/// # Note
///
/// This function determines if `x` is a factor of `y` by checking if `y` is evenly divisible by `x`
/// (i.e., `y % x == 0`).
pub fn is_factor(x: u32, y: u32) -> bool {
    y % x == 0
}

/// Checks if a positive integer `x` is a multiple of another positive integer `y`.
/// 
/// # Challenge
/// 
/// Write a program that determines whether one positive integer is a multiple of another.
///
/// # Description
/// 
/// Checks if a positive integer `x` is a multiple of another positive integer `y`.
///
/// A multiple of `y` is a positive integer `x` where `x` is evenly divisible by `y` (i.e., `x % y == 0`).
///
/// # Arguments
///
/// * `x` - The positive integer to determine whether it is a multiple of `y` or not.
/// * `y` - The positive integer for which the multiple check of `x` is performed.
///
/// # Returns
///
/// `true` if `x` is a multiple of `y`, `false` otherwise.
///
/// # Examples
///
/// ```rust
/// use ladderz::pre_algebra::unit1::is_multiple;
///
/// fn main() {
///     assert!(is_multiple(16, 2)); // 16 is a multiple of 2
///     assert!(!is_multiple(16, 3)); // 16 is not a multiple of 3
/// }
/// ```
pub fn is_multiple(x: u32, y: u32) -> bool {
    x % y == 0
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

    #[test]
    fn test_is_factor() {
        let result: bool = true;
        let expected: bool = is_factor(2, 10);
        assert_eq!(result, expected);

        let result_2: bool = false;
        let expected_2: bool =  is_factor(3, 10);
        assert_eq!(result_2, expected_2);
    }

    #[test]
    fn test_is_multiple() {
        let result: bool = true;
        let expected: bool = is_multiple(10, 2);
        assert_eq!(result, expected);

        let result_2: bool = false;
        let expected_2: bool = is_multiple(11, 2);
        assert_eq!(result_2, expected_2);
    }
}
