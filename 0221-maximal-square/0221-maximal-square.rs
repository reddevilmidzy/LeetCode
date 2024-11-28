impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();

        let mut dp : Vec<Vec<i32>> = vec![vec![0; m + 1]; n + 1];
        let mut res = 0;

        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] == '1' {
                    dp[i+1][j+1] = dp[i][j].min(dp[i+1][j]).min(dp[i][j+1]) + 1;
                    res = res.max(dp[i+1][j+1]);
                }
            }
        }
        res * res
    }
}
