pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut merged = Vec::with_capacity(nums1.len() + nums2.len());
        merged.extend(nums1);
        merged.extend(nums2);
        merged.sort_unstable();
        let n = merged.len();
        if n % 2 == 1 {
            merged[n / 2] as f64
        } else {
            (merged[n / 2 - 1] as f64 + merged[n / 2] as f64) / 2.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1,3], vec![2]), 2.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![1,2], vec![3,4]), 2.5);
    }
}

fn main() {
    println!("{}", Solution::find_median_sorted_arrays(vec![1,2], vec![3,4]));
}
