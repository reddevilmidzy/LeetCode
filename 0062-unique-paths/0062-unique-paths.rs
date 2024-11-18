impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut dp : Vec<Vec<i32>> = vec![vec![0; (n + 1) as usize]; (m + 1) as usize];

        dp[1][1] = 1;

        for i in 1..=m as usize {
            for j in 1..=n as usize {
                dp[i][j] += dp[i-1][j] + dp[i][j-1];
            }
        }

        dp[m as usize][n as usize]
    }
}
