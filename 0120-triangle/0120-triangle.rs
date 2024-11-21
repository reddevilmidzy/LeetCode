impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n : usize = triangle.len();
        let mut dp : Vec<Vec<i32>> = Vec::with_capacity(n);

        for i in 1..=n {
            let mut tmp = vec![0; i];
            dp.push(tmp);
        }
        dp[0] = triangle[0].clone();

        for i in 1..n {
            for j in 0..dp[i].len() {
                if j == 0 {
                    dp[i][j] = dp[i-1][j] + triangle[i][j]
                } else if j == dp[i].len() - 1 {
                    dp[i][j] = dp[i-1][j-1] + triangle[i][j]
                } else {
                    dp[i][j] = dp[i-1][j-1].min(dp[i-1][j]) + triangle[i][j]
                }
            }
        }
        *dp[n-1].iter().min().unwrap()
    }
}
