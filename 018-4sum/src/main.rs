pub struct Solution;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort();
        let n = nums.len();
        let mut res = Vec::new();
        for i in 0..n {
            if i>0 && nums[i]==nums[i-1] {continue;}
            for j in i+1..n {
                if j>i+1 && nums[j]==nums[j-1] {continue;}
                let mut l=j+1;
                let mut r=n-1;
                while l<r {
                    let sum=nums[i]+nums[j]+nums[l]+nums[r];
                    if sum==target {
                        res.push(vec![nums[i],nums[j],nums[l],nums[r]]);
                        l+=1; r-=1;
                        while l<r && nums[l]==nums[l-1] {l+=1;}
                        while l<r && nums[r]==nums[r+1] {r-=1;}
                    } else if sum<target {l+=1;} else {r-=1;}
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests{use super::*;#[test]fn test(){let mut r=Solution::four_sum(vec![1,0,-1,0,-2,2],0);r.sort();assert_eq!(r,vec![vec![-2,-1,1,2],vec![-2,0,0,2],vec![-1,0,0,1]]);}}

fn main(){println!("{:?}",Solution::four_sum(vec![1,0,-1,0,-2,2],0));}
