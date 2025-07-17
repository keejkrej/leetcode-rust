pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 || s.len() <= num_rows as usize {
            return s;
        }
        let mut rows = vec![String::new(); num_rows as usize];
        let mut idx = 0;
        let mut step = 1;
        for ch in s.chars() {
            rows[idx].push(ch);
            if idx == 0 { step = 1; }
            if idx == num_rows as usize -1 { step = -1; }
            idx = (idx as i32 + step) as usize;
        }
        rows.concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        assert_eq!(Solution::convert("PAYPALISHIRING".to_string(),3),"PAHNAPLSIIGYIR");
    }
}

fn main() {
    println!("{}", Solution::convert("PAYPALISHIRING".to_string(),3));
}
