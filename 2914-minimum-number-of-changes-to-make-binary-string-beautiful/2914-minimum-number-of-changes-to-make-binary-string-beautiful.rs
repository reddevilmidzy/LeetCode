impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let mut c = s.chars();
        let mut res: i32 = 0;
        while let Some(x) = c.next() {
            if let Some(y) = c.next() {
                if x != y {
                    res += 1;
                }
            }
        }
        res
    }
}
