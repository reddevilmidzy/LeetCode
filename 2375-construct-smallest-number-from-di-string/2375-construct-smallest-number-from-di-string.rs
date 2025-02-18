impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        use std::fmt::Write;

        let mut res = String::new();
        let n = pattern.len();
        let mut stk = Vec::new();
        let mut st = pattern.as_bytes();

        for idx in 0..=n {
            stk.push(idx + 1);

            if idx == n || st[idx] == b'I' {
                while !stk.is_empty() {
                    write!(res, "{}", stk.pop().unwrap());
                }
            }
        }
        
        res
    }
}