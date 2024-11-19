impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let n = obstacle_grid.len();
        let m = obstacle_grid[0].len();

        let mut dp : Vec<Vec<i32>> = vec![vec![0; m + 1]; n + 1];

        if obstacle_grid[0][0] == 0 {
            dp[1][1] = 1;
        }

        for i in 1..=n {
            for j in 1..=m {
                if obstacle_grid[i-1][j-1] == 1 {
                    continue;
                }

                dp[i][j] += dp[i-1][j] + dp[i][j-1];
            }
        }

        dp[n][m]
    }
}
