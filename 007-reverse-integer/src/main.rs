pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut num = x;
        let mut rev: i64 = 0;
        while num != 0 {
            rev = rev * 10 + (num % 10) as i64;
            num /= 10;
        }
        if rev > i32::MAX as i64 || rev < i32::MIN as i64 {0} else {rev as i32}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        assert_eq!(Solution::reverse(123),321);
        assert_eq!(Solution::reverse(-123),-321);
        assert_eq!(Solution::reverse(1534236469),0); // overflow
    }
}

fn main() {println!("{}",Solution::reverse(123));}
