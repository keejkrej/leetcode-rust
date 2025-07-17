pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        fn helper(s: &[u8], p: &[u8]) -> bool {
            if p.is_empty() { return s.is_empty(); }
            let first_match = !s.is_empty() && (p[0] == b'.' || p[0] == s[0]);
            if p.len() >=2 && p[1] == b'*' {
                helper(s, &p[2..]) || (first_match && helper(&s[1..], p))
            } else {
                first_match && helper(&s[1..], &p[1..])
            }
        }
        helper(s.as_bytes(), p.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        assert!(!Solution::is_match("aa".to_string(), "a".to_string()));
        assert!(Solution::is_match("aa".to_string(), "a*".to_string()));
        assert!(Solution::is_match("ab".to_string(), ".*".to_string()));
    }
}

fn main(){println!("{}",Solution::is_match("aa".to_string(), "a*".to_string()));}
