impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n : usize = nums.len();
        let mut dp = vec![0; n + 1];
        dp[0] = nums[0];
        if n == 1 {
            return dp[0];
        }
        dp[1] = nums[1].max(nums[0]);
        for i in 2..n {
            dp[i] = dp[i-1].max(dp[i-2] + nums[i]);
        }
        dp[n-1].max(dp[n-2])
    }
}