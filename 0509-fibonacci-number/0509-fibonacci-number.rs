impl Solution {
    pub fn fib(n: i32) -> i32 {
        let m : usize = 30;
        let mut dp = vec![0; m + 1];
        
        dp[0] = 0;
        dp[1] = 1;

        for i in 2..=n {
            dp[i as usize] = dp[i as usize - 1] + dp[i as usize - 2];
        }

        dp[n as usize] as i32
    }
}
