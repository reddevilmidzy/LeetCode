impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        let mut res = 1;
        let n = n as i64;
        for i in 2..=n {
            res += 4 * (i - 1);
        }

        res
    }
}
