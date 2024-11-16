impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut dp : Vec<i32> = vec![0; (n as usize).max(2) + 1];
        dp[1] = 1;
        dp[2] = 2;

        for i in 3..=n {
            dp[i as usize] = dp[i as usize -1] + dp[i as usize -2];
        }
        dp[n as usize] as i32
    }
}