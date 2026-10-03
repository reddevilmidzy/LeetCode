impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let n = s.len();
        let s = s.as_bytes();
        let mut stk = Vec::new();
        stk.push(-1);
        let mut res = 0;

        for i in 0..n {
            if s[i] == b'(' {
                stk.push(i as i32);
            } else {
                stk.pop();

                if let Some(j) = stk.last() {
                    res = res.max(i as i32 - j);
                } else {
                    stk.push(i as i32);
                }
            }
        }

        res
    }
}    
