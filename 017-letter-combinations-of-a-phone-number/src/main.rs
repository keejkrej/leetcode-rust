pub struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {return vec![];}
        let map = ["", "", "abc","def","ghi","jkl","mno","pqrs","tuv","wxyz"];
        let mut res = vec![String::new()];
        for d in digits.chars() {
            let letters = map[d.to_digit(10).unwrap() as usize];
            let mut next = Vec::new();
            for prefix in &res {
                for c in letters.chars() {
                    let mut p = prefix.clone();
                    p.push(c);
                    next.push(p);
                }
            }
            res = next;
        }
        res
    }
}

#[cfg(test)]
mod tests{use super::*;#[test]fn test(){assert_eq!(Solution::letter_combinations("23".to_string()),vec!["ad","ae","af","bd","be","bf","cd","ce","cf"].iter().map(|s|s.to_string()).collect::<Vec<_>>());}}

fn main(){println!("{:?}",Solution::letter_combinations("23".to_string()));}
