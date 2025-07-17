use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut set = HashSet::new();
        let mut left = 0;
        let mut max_len = 0;
        for right in 0..bytes.len() {
            while set.contains(&bytes[right]) {
                set.remove(&bytes[left]);
                left += 1;
            }
            set.insert(bytes[right]);
            max_len = max_len.max(right - left + 1);
        }
        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
    }
}

fn main() {
    let s = "pwwkew".to_string();
    println!("{}", Solution::length_of_longest_substring(s));
}
