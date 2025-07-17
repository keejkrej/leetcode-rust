pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {return String::new();}
        let mut prefix = strs[0].clone();
        for s in strs.iter().skip(1) {
            while !s.starts_with(&prefix) {
                prefix.pop();
                if prefix.is_empty() {return prefix;}
            }
        }
        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples(){
        assert_eq!(Solution::longest_common_prefix(vec!["flower","flow","flight"].iter().map(|s|s.to_string()).collect()),"fl");
    }
}

fn main(){println!("{}",Solution::longest_common_prefix(vec!["dog","racecar","car"].iter().map(|s|s.to_string()).collect()));}
