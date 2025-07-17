use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let map: HashMap<char, i32> = [('I',1),('V',5),('X',10),('L',50),('C',100),('D',500),('M',1000)].iter().cloned().collect();
        let bytes: Vec<char> = s.chars().collect();
        let mut res = 0;
        let mut i = 0;
        while i < bytes.len() {
            let val = map[&bytes[i]];
            if i+1 < bytes.len() && map[&bytes[i+1]] > val {
                res += map[&bytes[i+1]] - val;
                i +=2;
            } else {
                res += val;
                i +=1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples(){
        assert_eq!(Solution::roman_to_int("III".to_string()),3);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()),58);
    }
}

fn main(){println!("{}",Solution::roman_to_int("MCMXCIV".to_string()));}
