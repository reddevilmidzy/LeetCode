impl Solution {
    pub fn can_cross(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;
        let n = nums.len();
        let mut dp = vec![HashSet::new(); n];
        
        if nums[1] != 1 {
            return false;
        }
        dp[1] = HashSet::from([1i32]);

        for i in 1..n {
            let (left, right) = dp.split_at_mut(i + 1);

            let cur = &left[i];

            for j in i + 1..n {
                let nxt = &mut right[j - i - 1];

                for &k in cur {
                    if (nums[i] + k).abs_diff(nums[j]) <= 1 {
                        nxt.insert(nums[j] - nums[i]);
                    }
                }
            }
        }

        !dp[n - 1].is_empty()
    }
}
