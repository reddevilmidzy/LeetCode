impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut cur = 0;

        for val in values {
            res = res.max(cur + val);
            cur = cur.max(val) - 1;
        }

        res
    }
}