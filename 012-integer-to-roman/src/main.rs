pub struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let vals = [1000,900,500,400,100,90,50,40,10,9,5,4,1];
        let syms = ["M","CM","D","CD","C","XC","L","XL","X","IX","V","IV","I"];
        let mut n = num;
        let mut res = String::new();
        for i in 0..vals.len() {
            while n >= vals[i] {
                n -= vals[i];
                res.push_str(syms[i]);
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
        assert_eq!(Solution::int_to_roman(3),"III");
        assert_eq!(Solution::int_to_roman(58),"LVIII");
    }
}

fn main(){println!("{}",Solution::int_to_roman(1994));}
