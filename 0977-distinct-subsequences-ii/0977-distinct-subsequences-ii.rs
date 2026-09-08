impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        const MOD: i32 = 10i32.pow(9) + 7;
        let mut res = 0;
        let mut dp = vec![1; 26];

        let s = s.as_bytes();
        for i in 0..s.len() {
            let cur = dp[(s[i] - b'a') as usize];
            res += cur;
            res %= MOD;

            dp[(s[i] - b'a') as usize] = 0;
            for i in 0..26 {
                dp[i] += cur;
                dp[i] %= MOD;
            }
        }

        res
    }
}
