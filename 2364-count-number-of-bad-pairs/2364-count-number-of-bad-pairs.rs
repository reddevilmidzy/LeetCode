use std::collections::HashMap;

impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let mut res = 0;
        let n = nums.len();
        let mut diff_cnt = HashMap::new();

        for i in 0..n {
            let diff: i32 = i as i32 - nums[i] as i32;
            let good: i32 = *diff_cnt.get(&diff).unwrap_or(&0);
            
            res += i as i64 - good as i64;
            
            diff_cnt.insert(diff, good + 1);
        }
        
        res
    }
}