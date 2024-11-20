impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        let n: usize = s.len();
        let bs = s.as_bytes();
        let mut cnt = [0, 0, 0];

        for c in bs {
            cnt[*c as usize - 97] += 1;
        }
        
        if *cnt.iter().min().unwrap() < k {
            return -1;
        }

        let mut l = 0;
        let mut res = i32::MAX;

        for r in 0..n {
            cnt[bs[r] as usize - 97] -= 1;
            while *cnt.iter().min().unwrap() < k {
                cnt[bs[l] as usize - 97] += 1;
                l += 1;
            }

            res = res.min((n - (r - l + 1)) as i32);
        }
        res
    }
}
