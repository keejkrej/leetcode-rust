pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // Negative numbers are not palindromes
        if x < 0 {
            return false;
        }
        let s = x.to_string();
        s.chars().eq(s.chars().rev())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_numbers() {
        assert!(Solution::is_palindrome(121));
        assert!(Solution::is_palindrome(0));
        assert!(Solution::is_palindrome(1221));
    }

    #[test]
    fn test_non_palindrome_numbers() {
        assert!(!Solution::is_palindrome(-121));
        assert!(!Solution::is_palindrome(10));
        assert!(!Solution::is_palindrome(123));
    }
}

fn main() {
    let test_cases = vec![
        (121, true),
        (-121, false),
        (10, false),
        (1221, true),
        (123, false),
        (0, true),
    ];

    for (input, expected) in test_cases {
        let result = Solution::is_palindrome(input);
        println!("Input: {}, Result: {}, Expected: {}", input, result, expected);
    }
}
