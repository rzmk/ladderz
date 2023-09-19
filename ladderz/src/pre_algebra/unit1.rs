use std::collections::HashSet;

// TODO: Implement negative integers
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_factor_pairs_1() {
        let result: HashSet<(u32, u32)> = get_factor_pairs(1);
        let expected: HashSet<(u32, u32)> = [(1, 1)].into();
        assert_eq!(result, expected);
    }
}
