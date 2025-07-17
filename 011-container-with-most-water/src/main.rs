pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, height.len()-1);
        let mut ans = 0;
        while l < r {
            let area = (r-l) as i32 * height[l].min(height[r]);
            ans = ans.max(area);
            if height[l] < height[r] {l+=1;} else {r-=1;}
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples(){
        assert_eq!(Solution::max_area(vec![1,8,6,2,5,4,8,3,7]),49);
    }
}

fn main(){println!("{}",Solution::max_area(vec![1,8,6,2,5,4,8,3,7]));}
