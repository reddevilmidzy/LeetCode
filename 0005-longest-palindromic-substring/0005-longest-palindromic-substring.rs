impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let n = s.len();
        let mut dp = vec![vec![false; n]; n];
        let mut left = 0;
        let mut right = 0;
        let mut cs = s.as_bytes();
        for i in 0..n {
            dp[i][i] = true;
        }
        
        for i in 0..n-1 {
            if cs[i] == cs[i+1] {
                dp[i][i+1] = true;
                left = i;
                right = i + 1;
            }
        }

        for k in 2..n {
            for i in 0..n-k {
                let j = i + k;
                if cs[i] == cs[j] && dp[i+1][j-1] {
                    dp[i][j] = true;
                    left = i;
                    right = j;
                }
            }
        }
        s.get(left..=right).unwrap().to_string()
    }
}