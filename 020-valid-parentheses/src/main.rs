pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '('|'['|'{' => stack.push(c),
                ')' => if stack.pop() != Some('(') {return false;},
                ']' => if stack.pop() != Some('[') {return false;},
                '}' => if stack.pop() != Some('{') {return false;},
                _ => {}
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests{use super::*;#[test]fn test(){assert!(Solution::is_valid("()[]{}".to_string()));assert!(!Solution::is_valid("(]".to_string()));}}

fn main(){println!("{}",Solution::is_valid("()[]{}".to_string()));}
