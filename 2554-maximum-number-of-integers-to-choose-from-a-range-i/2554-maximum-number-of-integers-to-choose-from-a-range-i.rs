use std::collections::HashSet;

impl Solution {
    pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        let nums : HashSet<i32> = banned.into_iter().collect();
        let mut cnt = 0;
        let mut tot = 0;

        for i in 1..=n {
            if nums.contains(&i) {
                continue;
            }
            if tot + i <= max_sum {
                tot += i;
                cnt += 1;
            } else {
                break;
            }
        }
        cnt
    }
}