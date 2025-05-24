use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_map = HashMap::new();

        for (current_idx, &num) in nums.iter().enumerate() {
            let complement = target - num;
            
            if let Some(&complement_idx) = num_map.get(&complement) {
                return vec![complement_idx as i32, current_idx as i32];
            }
            
            num_map.insert(num, current_idx);
        }
        
        panic!("No solution found according to problem constraints")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_case_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_two_sum_case_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_two_sum_case_3() {
        let nums = vec![3, 3];
        let target = 6;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }
}

fn main() {
    let test_cases = vec![
        (vec![2, 7, 11, 15], 9, vec![0, 1]),
        (vec![3, 2, 4], 6, vec![1, 2]),
        (vec![3, 3], 6, vec![0, 1]),
    ];

    for (nums, target, expected) in test_cases {
        let result = Solution::two_sum(nums.clone(), target);
        println!("Input: nums = {:?}, target = {}", nums, target);
        println!("Result: {:?}, Expected: {:?}\n", result, expected);
    }
}
