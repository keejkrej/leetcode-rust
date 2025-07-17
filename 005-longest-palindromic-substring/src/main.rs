pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() <= 1 { return s; }
        let bytes = s.as_bytes();
        let mut start = 0;
        let mut end = 0;
        for i in 0..bytes.len() {
            let len1 = Self::expand(&bytes, i as i32, i as i32);
            let len2 = Self::expand(&bytes, i as i32, i as i32 + 1);
            let len = len1.max(len2);
            if len > (end - start) as i32 {
                start = i as i32 - (len - 1) / 2;
                end = i as i32 + len / 2;
            }
        }
        s[start as usize..=end as usize].to_string()
    }

    fn expand(s: &[u8], mut l: i32, mut r: i32) -> i32 {
        while l >= 0 && (r as usize) < s.len() && s[l as usize] == s[r as usize] {
            l -= 1;
            r += 1;
        }
        r - l - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let res = Solution::longest_palindrome("babad".to_string());
        assert!(res == "bab" || res == "aba");
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
    }
}

fn main() {
    println!("{}", Solution::longest_palindrome("babad".to_string()));
}
