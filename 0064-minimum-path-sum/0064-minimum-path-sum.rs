impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n : usize = grid.len();
        let m : usize = grid[0].len();

        let mut dp : Vec<Vec<i32>> = vec![vec![100_000; m + 1]; n + 1];
        dp[0][1] = 0;
        dp[1][0] = 0;
        for i in 1..=n {
            for j in 1..=m {
                dp[i][j] = dp[i-1][j].min(dp[i][j-1]) + grid[i-1][j-1];
            }
        }

        dp[n][m] 
    }
}
