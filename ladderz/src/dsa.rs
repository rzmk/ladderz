use std::collections::{HashMap, HashSet};

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::<i32>::new();
    for &num in nums.iter() {
        if seen.contains(&num) {
            return true;
        }
        seen.insert(num);
    }
    false
}

pub fn is_anagram(a: String, b: String) -> bool {
    let mut letters = HashMap::new();
    for c in a.chars() {
        if let Some(&value) = letters.get(&c) {
            letters.insert(c, value + 1);
        } else {
            letters.insert(c, 1);
        }
    }
    for c in b.chars() {
        if let Some(&value) = letters.get(&c) {
            if value - 1 < 0 {
                return false;
            }
            letters.insert(c, value - 1);
        } else {
            return false;
        }
    }
    for (_, &count) in letters.iter() {
        if count > 0 {
            return false;
        }
    }
    true
}

pub fn is_anagram2(a: String, b: String) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut letters = HashMap::new();
    for c in a.chars() {
        *letters.entry(c).or_default() += 1;
    }
    for c in b.chars() {
        *letters.entry(c).or_default() -= 1;
    }

    letters.into_values().all(|c: i32| c == 0)
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        if seen.contains_key(num) {
            return vec![i as i32, seen[num]];
        }
        let diff = target - num;
        seen.insert(diff, i as i32);
    }
    vec![0, 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        let result = contains_duplicate(vec![1, 2, 3, 2]);
        let expected = true;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_is_anagram() {
        let result = is_anagram("marc".to_owned(), "cram".to_owned());
        let expected = true;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_two_sum() {
        let result: HashSet<i32> = HashSet::from_iter(two_sum(vec![2, 3, 8, 5], 8));
        let expected: HashSet<i32> = HashSet::from_iter(vec![1, 3]);
        assert_eq!(result, expected);
    }
}
