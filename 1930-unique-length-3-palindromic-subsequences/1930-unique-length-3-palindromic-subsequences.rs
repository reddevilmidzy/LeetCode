use std::collections::HashSet;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let n: usize = s.len();
        let mut first = vec![n; 26];
        let mut last = vec![n; 26];
        let mut res = 0;
        let s = s.as_bytes();

        for (i, &c) in s.iter().enumerate() {
            let idx = (c - b'a') as usize;
            if first[idx] == n {
                first[idx] = i;
            }
            last[idx] = i;
        }

        for i in 0..26 {
            if first[i] == n {
                continue;
            }
            let mut tmp = HashSet::new();
            for j in first[i]+1..last[i] {
                tmp.insert(s[j]);
            }
            res += tmp.len();
        }

        res as i32
    }
}