impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let n : usize = matrix.len();
        let m : usize = matrix[0].len();

        let mut dp : Vec<Vec<i32>> = vec![vec![1_000_000; m]; n];
        for j in 0..m {
            dp[0][j] = matrix[0][j];
        }
        for i in 1..n {
            for j in 0..m {
                if j != 0 {
                    dp[i][j] = dp[i][j].min(dp[i-1][j-1] + matrix[i][j]);
                }
                if j != m-1 {
                    dp[i][j] = dp[i][j].min(dp[i-1][j+1] + matrix[i][j]);
                }
                dp[i][j] = dp[i][j].min(dp[i-1][j] + matrix[i][j]);
            }
        }
        *dp[n-1].iter().min().unwrap()
    }
}
