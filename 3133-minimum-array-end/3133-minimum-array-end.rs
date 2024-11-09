impl Solution {
    pub fn min_end(mut n: i32, mut x: i32) -> i64 {
        let mut ans :i64 = 0;
        n -= 1;
        let mut i = 0;

        while ((n | x) != 0) {
            ans += (((n & 1) | (x & 1)) as i64) << i;
            if ((x & 1) == 0) {
                n >>= 1;
            }
            x >>= 1;
            i += 1;
        }

        ans
    }
}
