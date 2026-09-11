impl Solution {
    pub fn is_match(s: String, p: String) -> bool {

        let n = s.len();
        let m = p.len();
        // dp[i][j]  = s[i] ì p[j] ê° ê°ë¥íì§
        let mut dp = vec![vec![false; m + 1]; n + 1];
        dp[0][0] = true;
        let s = s.as_bytes();
        let p = p.as_bytes();
        for i in 0..=n {
            for j in 0..m {
                if !dp[i][j] {
                    continue;
                }

                match p[j] {
                    b'*' => {
                        // *ë¥¼ ê·¸ë¥ ëì
                        dp[i][j + 1] = true;

                        // *ê° íë ì²´í¬
                        if i < n {
                            dp[i + 1][j] = true;
                        }
                    }
                    b'?' => {
                        if i < n {
                            dp[i + 1][j + 1] = true;
                        }
                    }
                    c => {
                        if i < n && s[i] == c {
                            dp[i + 1][j + 1] = true;
                        }
                    }
                }

            }
        }
        dp[n][m]
    }
}
