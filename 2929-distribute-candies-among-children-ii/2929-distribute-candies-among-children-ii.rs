impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i64 {
        let mut res = 0;
        for i in 0..=limit.min(n) {
            res += 0.max(limit.min(n - i) - 0.max(n - i - limit) + 1) as i64;
        }
        res
    }
}
