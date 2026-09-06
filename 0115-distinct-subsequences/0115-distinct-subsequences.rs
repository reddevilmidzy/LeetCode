impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let n = s.len();
        let m = t.len();
        let s = s.as_bytes();
        let t = t.as_bytes();

        // dp[i][j] = s[i]ë²ì§¸ ìì ê¹ì§ ì¬ì©íê³ , t[j] ê¹ì§ ì§íëìì ë
        let mut dp = vec![vec![0; m + 1]; n + 1];
        for i in 0..n {
            dp[i][0] = 1;
        }

        for i in 1..=n {
            for j in 1..=m {
                if s[i - 1] == t[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j];
                } else {
                    dp[i][j] = dp[i - 1][j];
                }
            }
        }

        dp[n][m]
    }
}
