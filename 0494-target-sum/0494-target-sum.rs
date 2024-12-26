impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let n : usize = nums.len();
        let m : i32 = 1000;
        let mut dp : Vec<Vec<i32>> = vec![vec![0; (m * 2 + 1) as usize]; n];

        dp[0][(nums[0] + m) as usize] = 1;
        dp[0][(m - nums[0]) as usize] += 1;

        for i in 1..n {
            for j in 0..=(m as usize * 2) {
                if j as i32 + nums[i] <= 2 * m && dp[i-1][(j as i32 + nums[i]) as usize] != 0 {
                    dp[i][j] += dp[i-1][(j as i32 + nums[i]) as usize];
                }
                if 0 <= j as i32 - nums[i] && dp[i-1][(j as i32 - nums[i]) as usize] != 0 {
                    dp[i][j] += dp[i-1][(j as i32 - nums[i]) as usize];
                }

            }
        }
        dp[n-1][(target + m) as usize]
    }
}
