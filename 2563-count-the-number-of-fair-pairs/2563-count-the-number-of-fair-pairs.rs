impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let n = nums.len();
        let mut res = 0;
        nums.sort_unstable();

        for i in 0..n {
            let left_val = lower - nums[i];
            let right_val = upper - nums[i];

            let left_idx = nums[i+1..].partition_point(|&x| x < left_val);
            let right_idx = nums[i+1..].partition_point(|&x| x <= right_val);

            res += (right_idx - left_idx) as i64;
        }

        res
    }
}
