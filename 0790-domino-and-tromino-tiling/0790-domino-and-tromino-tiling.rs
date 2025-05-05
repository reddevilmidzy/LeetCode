impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        // 1 = 1
        // 2 = 2 
        // 3 = 5
        // 4 =  => dp[n-1] * 2 + dp[n-3]
        // 
        let modulo = 1_000_000_007;
        let n = n as usize;
        let mut dp = vec![0i64; n.max(3) + 1];
        dp[1] = 1;
        dp[2] = 2;
        dp[3] = 5;

        for i in 4..=n {
            dp[i] = dp[i-1] * 2 + dp[i-3];
            dp[i] %= modulo;
        }

        dp[n] as i32
    }
}
