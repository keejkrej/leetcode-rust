pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut chars = s.trim_start().chars().peekable();
        let mut sign = 1;
        if let Some(&c) = chars.peek() {
            if c == '-' {sign = -1; chars.next();}
            else if c == '+' {chars.next();}
        }
        let mut num: i64 = 0;
        while let Some(c) = chars.peek() {
            if !c.is_ascii_digit() { break; }
            num = num*10 + (c.to_digit(10).unwrap() as i64);
            chars.next();
            if sign == 1 && num > i32::MAX as i64 {return i32::MAX;}
            if sign == -1 && -num < i32::MIN as i64 {return i32::MIN;}
        }
        (sign*num as i64).clamp(i32::MIN as i64, i32::MAX as i64) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        assert_eq!(Solution::my_atoi("42".to_string()),42);
        assert_eq!(Solution::my_atoi("   -42".to_string()),-42);
        assert_eq!(Solution::my_atoi("4193 with words".to_string()),4193);
    }
}

fn main() {println!("{}",Solution::my_atoi("42".to_string()));}
