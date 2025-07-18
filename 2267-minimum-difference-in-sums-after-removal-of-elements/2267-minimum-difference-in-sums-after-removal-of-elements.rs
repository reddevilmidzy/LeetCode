use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i64 {
        let n = nums.len() / 3;

        let mut res = i64::MAX;

        let mut min_hq = BinaryHeap::new();
        let mut max_hq = BinaryHeap::new();

        let mut min_val = 0;
        for i in 0..n {
            max_hq.push(nums[i]);
            min_val += nums[i] as i64;
        }

        let mut max_dp = vec![0; n * 3];

        let mut max_val = 0;
        for i in (n..nums.len()).rev() {
            max_val += nums[i] as i64;
            min_hq.push(-nums[i]);

            if i < 2 * n {
                let val = -min_hq.pop().unwrap();
                max_val -= val as i64;
            }
            max_dp[i] = max_val;
        }

        // println!("{max_dp:?}");
        res = res.min(min_val - max_dp[n]);

        for i in n..2*n {
            max_hq.push(nums[i]);
            let val = max_hq.pop().unwrap();
            min_val += nums[i] as i64;
            min_val -= val as i64;

            res = res.min(min_val - max_dp[i + 1]);
        }

        res
    }
}
