impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut res = 1;
        let n = nums.len();

        let mut inc_dp = 1;
        let mut dec_dp = 1;

        for i in 1..n {
            if nums[i-1] < nums[i] {
                inc_dp += 1;
            } else {
                inc_dp = 1;
            }
            if nums[i-1] > nums[i] {
                dec_dp += 1;
            } else {
                dec_dp = 1;
            }
            res = res.max(inc_dp).max(dec_dp);
        }

        res
    }
}