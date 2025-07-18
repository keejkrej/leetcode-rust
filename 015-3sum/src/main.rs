pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut res = Vec::new();
        for i in 0..nums.len() {
            if i>0 && nums[i]==nums[i-1] {continue;}
            let mut l=i+1;
            let mut r=nums.len()-1;
            while l<r {
                let sum=nums[i]+nums[l]+nums[r];
                if sum==0 {
                    res.push(vec![nums[i],nums[l],nums[r]]);
                    l+=1; r-=1;
                    while l<r && nums[l]==nums[l-1] {l+=1;}
                    while l<r && nums[r]==nums[r+1] {r-=1;}
                } else if sum<0 {l+=1;} else {r-=1;}
            }
        }
        res
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_example(){
        let mut res = Solution::three_sum(vec![-1,0,1,2,-1,-4]);
        res.sort();
        assert_eq!(res, vec![vec![-1,-1,2],vec![-1,0,1]]);
    }
}

fn main(){println!("{:?}",Solution::three_sum(vec![-1,0,1,2,-1,-4]));}
