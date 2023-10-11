use std::collections::{HashMap, HashSet};

/// Finds all factor pairs for a positive integer `n`.
///
/// A factor pair is a pair of positive integers
/// `(a, b)` where `a` and `b` are both factors of `n` (i.e., `a * b == n`).
///
/// This function calculates and returns a `HashSet<(u32, u32)>` containing all unique factor pairs
/// of the input positive integer `n`.
///
/// # Examples
///
/// ```rust
/// use std::collections::HashSet;
/// use ladderz::pre_algebra::get_factor_pairs;
///
/// let result_pairs = get_factor_pairs(12);
/// let expected_pairs: HashSet<(u32, u32)> = [(1, 12), (2, 6), (3, 4)].into();
/// assert_eq!(result_pairs, expected_pairs);
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
/// A factor of `n` is a positive integer `a` where
/// `n` is evenly divisible by `a` (i.e., `n % a == 0`).
///
/// This function calculates and returns a `HashSet<u32>` containing all unique factors
/// of the input positive integer `n`.
///
/// # Examples
///
/// ```rust
/// use std::collections::HashSet;
/// use ladderz::pre_algebra::get_factors;
///
/// let result_factors = get_factors(16);
/// let expected_factors: HashSet<u32> = [1, 2, 4, 8, 16].into();
/// assert_eq!(result_factors, expected_factors);
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
/// A factor of `y` is a positive integer `x` where `y` is evenly divisible by `x` (i.e., `y % x == 0`).
///
/// # Examples
///
/// ```rust
/// use ladderz::pre_algebra::is_factor;
///
/// assert!(is_factor(2, 16)); // 2 is a factor of 16
/// assert!(!is_factor(3, 16)); // 3 is not a factor of 16
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
/// A multiple of `y` is a positive integer `x` where `x` is evenly divisible by `y` (i.e., `x % y == 0`).
///
/// # Examples
///
/// ```rust
/// use ladderz::pre_algebra::is_multiple;
///
/// assert!(is_multiple(16, 2)); // 16 is a multiple of 2
/// assert!(!is_multiple(16, 3)); // 16 is not a multiple of 3
/// ```
pub fn is_multiple(x: u32, y: u32) -> bool {
    x % y == 0
}

/// Finds all the multiples of a positive integer `n` up to and including `end` (in the range [n, end]).
///
/// A multiple of `n` is a positive integer `num` where `num` is evenly divisible by `n` (i.e., `num % n == 0`).
///
/// # Examples
///
/// ```rust
/// use ladderz::pre_algebra::get_multiples_in_range;
/// use std::collections::HashSet;
///
/// let result: HashSet<u32> = get_multiples_in_range(2, 10);
/// let expected: HashSet<u32> = [2, 4, 6, 8, 10].into();
/// assert_eq!(result, expected);
/// ```
pub fn get_multiples_in_range(n: u32, end: u32) -> HashSet<u32> {
    let mut multiples: HashSet<u32> = HashSet::new();

    for num in (n..end + 1).step_by(n as usize) {
        multiples.insert(num);
    }
    multiples
}

/// Checks if a positive integer `n` is a prime number.
///
/// A prime number is a positive integer greater than 1 that is
/// not evenly divisible by any positive integer other than 1 and itself.
///
/// # Examples
///
/// ```rust
/// use ladderz::pre_algebra::is_prime;
///
/// assert!(!is_prime(1)); // 1 is not a prime number
/// assert!(is_prime(2)); // 2 is a prime number
/// assert!(!is_prime(4)); // 4 is not a prime number
/// assert!(is_prime(7)); // 7 is a prime number
/// ```
///
/// # Note
///
/// This function returns `false` for values of `n` less than 2 because, by definition, prime
/// numbers must be greater than 1.
pub fn is_prime(n: u32) -> bool {
    if n == 1 {
        return false;
    };
    for num in 2..n {
        if n % num == 0 {
            return false;
        }
    }
    true
}

/// Checks if a positive integer `n` is a composite number.
///
/// A composite number is a positive integer that is evenly divisible
/// by a positive integer other than 1 and itself.
///
/// # Examples
///
/// ```rust
/// use ladderz::pre_algebra::is_composite;
///
/// assert!(!is_composite(1)); // 1 is not a composite number
/// assert!(!is_composite(2)); // 2 is not a composite number
/// assert!(is_composite(4)); // 4 is a composite number
/// assert!(!is_composite(7)); // 7 is not a composite number
/// ```
pub fn is_composite(n: u32) -> bool {
    for num in 1..n {
        if n % num == 0 && num != 1 && num != n {
            return true;
        }
    }
    false
}

/// Write a program that finds all prime numbers in the range [start, end] within the natural numbers.
///
/// A prime number is a positive integer greater than 1 that is
/// not evenly divisible by any positive integer other than 1 and itself.
///
/// # Examples
///
/// ```rust
/// use ladderz::pre_algebra::get_primes_in_range;
/// use std::collections::HashSet;
///
/// let result: HashSet<u32> = get_primes_in_range(2, 10);
/// let expected: HashSet<u32> = [2, 3, 5, 7].into();
/// assert_eq!(result, expected);
/// ```
pub fn get_primes_in_range(start: u32, end: u32) -> HashSet<u32> {
    let mut primes: HashSet<u32> = HashSet::new();

    for num in start..end + 1 {
        if is_prime(num) {
            primes.insert(num);
        }
    }
    primes
}

/// Write a program that determines the prime factorization of a positive integer `n`.
///
/// For example the prime factorization of 12 is 2<sup>2</sup> * 3<sup>1</sup>, and the output is a HashMap of the form
/// `[(2, 2), (3, 1)]` where the first element of each tuple is the prime factor and the second element is the exponent.
///
/// # Examples
///
/// ```rust
/// use ladderz::pre_algebra::get_prime_factorization;
/// use std::collections::HashMap;
///
/// let result: HashMap<u32, u32> = get_prime_factorization(12);
/// let expected: HashMap<u32, u32> = [(2, 2), (3, 1)].into();
/// assert_eq!(result, expected);
/// ```
pub fn get_prime_factorization(n: u32) -> HashMap<u32, u32> {
    if n == 1 {
        return HashMap::new();
    }

    let mut prime_factors: HashMap<u32, u32> = HashMap::new();
    let mut current_num: u32 = n;
    let primes_of_n: HashSet<u32> = get_primes_in_range(2, n);

    for prime in primes_of_n {
        while current_num % prime == 0 {
            prime_factors
                .entry(prime)
                .and_modify(|e| *e += 1)
                .or_insert(1);
            current_num /= prime;
        }
    }
    prime_factors
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
        let expected_2: bool = is_factor(3, 10);
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

    #[test]
    fn test_get_multiples_in_range() {
        let result: HashSet<u32> = get_multiples_in_range(2, 20);
        let expected: HashSet<u32> = [2, 4, 6, 8, 10, 12, 14, 16, 18, 20].into();
        assert_eq!(result, expected);

        let result_2: HashSet<u32> = get_multiples_in_range(5, 34);
        let expected_2: HashSet<u32> = [5, 10, 15, 20, 25, 30].into();
        assert_eq!(result_2, expected_2);

        let result_3: HashSet<u32> = get_multiples_in_range(7, 11);
        let expected_3: HashSet<u32> = [7].into();
        assert_eq!(expected_3, result_3);
    }

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(51));
        assert!(is_prime(23));
        assert!(!is_prime(9514));
    }

    #[test]
    fn test_is_composite() {
        assert!(!is_composite(1));
        assert!(!is_composite(2));
        assert!(!is_composite(3));
        assert!(is_composite(4));
        assert!(is_composite(8));
        assert!(is_composite(27));
        assert!(is_composite(51));
    }

    #[test]
    fn test_get_primes_in_range() {
        let result: HashSet<u32> = get_primes_in_range(2, 10);
        let expected: HashSet<u32> = [2, 3, 5, 7].into();
        assert_eq!(result, expected);

        let result_2: HashSet<u32> = get_primes_in_range(1, 10);
        let expected_2: HashSet<u32> = [2, 3, 5, 7].into();
        assert_eq!(result_2, expected_2);

        let result_3: HashSet<u32> = get_primes_in_range(1, 1);
        let expected_3: HashSet<u32> = [].into();
        assert_eq!(result_3, expected_3);

        let result_4: HashSet<u32> = get_primes_in_range(1, 50);
        let expected_4: HashSet<u32> =
            [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47].into();
        assert_eq!(result_4, expected_4);
    }

    #[test]
    fn test_get_prime_factorization() {
        let result: HashMap<u32, u32> = get_prime_factorization(1);
        let expected: HashMap<u32, u32> = HashMap::new();
        assert_eq!(result, expected);

        let result_2: HashMap<u32, u32> = get_prime_factorization(12);
        let expected_2: HashMap<u32, u32> = [(2, 2), (3, 1)].into();
        assert_eq!(result_2, expected_2);

        let result_3: HashMap<u32, u32> = get_prime_factorization(16);
        let expected_3: HashMap<u32, u32> = [(2, 4)].into();
        assert_eq!(result_3, expected_3);

        let result_4: HashMap<u32, u32> = get_prime_factorization(27);
        let expected_4: HashMap<u32, u32> = [(3, 3)].into();
        assert_eq!(result_4, expected_4);

        let result_5: HashMap<u32, u32> = get_prime_factorization(51);
        let expected_5: HashMap<u32, u32> = [(3, 1), (17, 1)].into();
        assert_eq!(result_5, expected_5);
    }
}
