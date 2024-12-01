impl Solution {

    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let n = s.len();
        let mut dp = vec![vec![-1; n]; n];
        let mut c = s.as_bytes();
        pub fn rec(mut dp: &mut Vec<Vec<i32>>, c : &[u8], i: usize, j: usize) -> i32 {
            if dp[i][j] != -1 {
                return dp[i][j];
            }
            if i > j {
                dp[i][j] = 0;
                return dp[i][j];
            }
            if i == j {
                dp[i][j] = 1;
                return dp[i][j];
            }
            if c[i] == c[j] {
                dp[i][j] = 2 + rec(&mut dp, c, i+1, j-1);
                return dp[i][j];
            }
            dp[i][j] = rec(&mut dp, c, i+1, j).max(rec(&mut dp, c, i, j-1));
            dp[i][j]
        }
        rec(&mut dp, c, 0, n - 1)
    }
}
