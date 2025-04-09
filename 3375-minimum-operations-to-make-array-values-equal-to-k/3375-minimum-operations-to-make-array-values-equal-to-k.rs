use std::collections::HashSet;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        for i in 0..n {
            if nums[i] < k {
                return -1;
            }
        }
        let nums: HashSet<i32> = nums.into_iter().collect();
        if nums.contains(&k) {
            return nums.len() as i32 - 1;
        }
        nums.len() as i32
    }
}
