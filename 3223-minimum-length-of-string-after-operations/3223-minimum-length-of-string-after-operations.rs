impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut res = 0;
        let mut cnt = vec![0; 26];

        for c in s.chars() {
            cnt[(c as u8 - 'a' as u8) as usize] += 1;
        }

        for i in 0..26 {
            if cnt[i] >= 3 {
                res += 2 - cnt[i] % 2;
            } else {
                res += cnt[i];
            }
        }
        res
    }
}