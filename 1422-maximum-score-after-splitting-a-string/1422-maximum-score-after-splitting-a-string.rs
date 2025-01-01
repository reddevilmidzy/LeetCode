impl Solution {
    pub fn max_score(s: String) -> i32 {
        let s = s.as_bytes();
        let n : usize = s.len();

        let mut left = 0;
        let mut right = s.iter().filter(|x| **x == 49).count();
        if s[0] == 48 {
            left += 1;
        } else {
            right -= 1;
        }
        let mut res = left + right;

        for i in 1..n-1 {
            if s[i] == 48 {
                left += 1;
            } else {
                right -= 1;
            }
            res = res.max(left + right);
        }
        
        res as i32
    }
}