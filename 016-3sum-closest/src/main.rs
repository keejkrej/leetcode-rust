pub struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let mut best = i32::MAX/2;
        for i in 0..nums.len() {
            let mut l=i+1;
            let mut r=nums.len()-1;
            while l<r {
                let sum=nums[i]+nums[l]+nums[r];
                if (sum-target).abs() < (best-target).abs() {best=sum;}
                if sum<target {l+=1;} else if sum>target {r-=1;} else {return target;}
            }
        }
        best
    }
}

#[cfg(test)]
mod tests{use super::*;#[test]fn test_example(){assert_eq!(Solution::three_sum_closest(vec![-1,2,1,-4],1),2);}}

fn main(){println!("{}",Solution::three_sum_closest(vec![-1,2,1,-4],1));}
