impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let n : usize = cost.len();
        let m : usize = 1_000;
        let mut dp = vec![0; m + 1];

        dp[0] = cost[0];
        dp[1] = cost[1];

        for i in 2..n {
            dp[i] = dp[i-1].min(dp[i-2]) + cost[i];
        }

        dp[n-1].min(dp[n-2])
    }
}
