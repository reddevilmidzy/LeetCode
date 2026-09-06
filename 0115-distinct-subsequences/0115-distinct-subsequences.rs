impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let n = s.len();
        let m = t.len();
        let s = s.as_bytes();
        let t = t.as_bytes();

        let mut dp = vec![0; m + 1];
        dp[0] = 1;

        for i in 1..=n {
            for j in (1..=m).rev() {
                if s[i - 1] == t[j - 1] {
                    dp[j] += dp[j - 1];
                }
            }
        }

        dp[m]
    }
}
