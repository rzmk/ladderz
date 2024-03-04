use std::collections::HashSet;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        let result = contains_duplicate(vec![1, 2, 3, 2]);
        let expected = true;
        assert_eq!(result, expected);
    }
}
