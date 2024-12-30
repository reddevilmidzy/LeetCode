const MOD : i32 = 1_000_000_007;

impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let low : usize = low as usize;
        let high : usize = high as usize;
        let zero : usize = zero as usize;
        let one : usize = one as usize;
        let mut dp : Vec<i32> = vec![0; high + 1];

        dp[zero] += 1;
        dp[one] += 1;
        let mut res = 0;

        for i in 1..=high {
            if zero <= i {
                dp[i] += dp[i - zero];
            }
            if one <= i {
                dp[i] += dp[i - one];
            }
            dp[i] %= MOD;
            if low <= i && i <= high {
                res += dp[i];
                res %= MOD;
            }
        }
        res
    }
}
