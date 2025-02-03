impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let n = nums.len();

        let mut inc_dp = vec![1; n];
        let mut dec_dp = vec![1; n];

        for i in 1..n {
            if nums[i-1] < nums[i] {
                inc_dp[i] = inc_dp[i-1] + 1;
            }
            if nums[i-1] > nums[i] {
                dec_dp[i] = dec_dp[i-1] + 1;
            }
        }

        for i in 0..n {
            res = res.max(inc_dp[i]).max(dec_dp[i]);
        }
        res
    }
}