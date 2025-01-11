impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        let mut cnt = vec![0; 26];
        for c in s.chars() {
            cnt[(c as u8 - 'a' as u8) as usize] += 1;
        }

        let mut min_val = 0;
        let mut max_val = 0;
        for i in 0..26 {
            if cnt[i] % 2 != 0 {
                min_val += 1;
            } 
            max_val += cnt[i];
        }
        min_val <= k && k <= max_val
    }
}